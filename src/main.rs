#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        // nested in another panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("GridNestedLeft").show_inside(ui, |ui| {
                ui.label("Left Panel");
            });
            egui::SidePanel::right("GridNestedRight").show_inside(ui, |ui| {
                ui.label("Right Panel");
            });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label("Central Panel");
            });
        });

        // Nested in window
        egui::Window::new("Test").show(ctx, |ui| {
            egui::SidePanel::left("WindowRight")
                .default_width(100.0)
                .width_range(10.0..=1000.0)
                .show_inside(ui, |ui| ui.label("Left Panel"));
            egui::SidePanel::right("WindowLeft")
                .default_width(100.0)
                .width_range(10.0..=1000.0)
                .show_inside(ui, |ui| ui.label("Right Panel"));
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label("Central Panel");
            });
        });
    })
}
