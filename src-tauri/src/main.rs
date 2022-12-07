#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            utils::read_image_base64,
            commands::upscale_single_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
