use opencv::prelude::*;
use opencv::videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY};
use opencv::highgui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <video file>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];

    let mut cap = VideoCapture::from_file(file_path, CAP_ANY)?;
    if !cap.is_opened()? {
        eprintln!("Error: Could not open video file.");
        return Ok(());
    }

    loop {
        let mut frame = Mat::default();

        cap.read(&mut frame)?;

        if frame.empty() {
            break;
        }

        highgui::imshow("Video", &frame)?;

        if highgui::wait_key(1)? == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}

