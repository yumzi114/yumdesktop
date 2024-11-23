use std::{default, fmt::format};
use serde::{Deserialize, Serialize};
use eframe::egui::{self, menu, Pos2, Rect, ViewportBuilder, ViewportInfo};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let windows = ViewportBuilder{
        title: Some(String::from("YumDesktop")),
        app_id: Some(String::from("YumDesktop")),
        clamp_size_to_monitor_size:Some(true),
        // fullsize_content_view: Some(true),
        titlebar_shown: Some(true),
        
        // resizable: Some(false),
        // fullscreen:Some(true),
        ..Default::default()
    };
    
    let native_options = eframe::NativeOptions{
        viewport:windows,
        ..Default::default()
    };
    eframe::run_native(
        "YumDesktop", 
        native_options, 
        Box::new(|cc| Ok(Box::new(YumDesktop::new(cc)))));
}
#[derive(Debug, EnumIter)]
enum MenuList {
    AppSetting(String),
    MQTT(String),
    DNS(String),
}
// #[derive(Default)]
struct YumDesktop {
    window_size:Rect,
    menu_list:Vec<MenuList>
}

impl YumDesktop {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self{
            window_size:Rect {
                min: Pos2::new(0 as f32, 0 as f32),
                max: Pos2::new(0 as f32, 0 as f32),
            },
            menu_list:vec![
                MenuList::AppSetting(format!("🖳 MAIN")),
                MenuList::MQTT(format!("📡 MQTT")),
                MenuList::DNS(format!("🔓 DNS"))
                ]
        }
    }
}

impl eframe::App for YumDesktop {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //Display config 
        ctx.request_repaint();
        self.window_size=ctx.screen_rect();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

        egui::CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                // let dd:Option<_> = self.menu_list.iter().find_map(|menu| match *menu {
                //     MenuList::MQTT(ref str)=>{Some(str)},
                //     _=>{None},
                // });
                ui.menu_button(self.menu_list.iter().find_map(|menu| match *menu {
                    MenuList::AppSetting(ref str)=>{Some(str)},
                    _=>{None},
                    }).unwrap(), |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
                ui.menu_button(self.menu_list.iter().find_map(|menu| match *menu {
                    MenuList::MQTT(ref str)=>{Some(str)},
                    _=>{None},
                    }).unwrap(), |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
                ui.menu_button(self.menu_list.iter().find_map(|menu| match *menu {
                    MenuList::DNS(ref str)=>{Some(str)},
                    _=>{None},
                    }).unwrap(), |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });
            
            ui.label(format!("{:?}",self.window_size).as_str());
            ui.label(format!("{:?}",self.menu_list).as_str());
       });
   }
}