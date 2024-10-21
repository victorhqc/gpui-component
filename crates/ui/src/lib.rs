mod colors;
mod event;
mod focusable;
mod icon;
mod root;
mod styled;
mod svg_img;
mod time;
mod title_bar;

pub mod accordion;
pub mod animation;
pub mod button;
pub mod button_group;
pub mod checkbox;
pub mod clipboard;
pub mod color_picker;
pub mod context_menu;
pub mod divider;
pub mod dock;
pub mod drawer;
pub mod dropdown;
pub mod history;
pub mod indicator;
pub mod input;
pub mod label;
pub mod link;
pub mod list;
pub mod modal;
pub mod notification;
pub mod popover;
pub mod popup_menu;
pub mod prelude;
pub mod progress;
pub mod radio;
pub mod resizable;
pub mod scroll;
pub mod skeleton;
pub mod slider;
pub mod switch;
pub mod tab;
pub mod table;
pub mod theme;
pub mod tooltip;
pub mod webview;

// re-export
pub use wry;

pub use crate::Disableable;
pub use event::InteractiveElementExt;
pub use focusable::FocusableCycle;
pub use root::{ContextModal, Root};
pub use styled::*;
pub use time::*;
pub use title_bar::*;

pub use colors::*;
pub use icon::*;
pub use svg_img::*;

use std::ops::Deref;

rust_i18n::i18n!("locales", fallback = "en");

/// Initialize the UI module.
///
/// This must be called before using any of the UI components.
/// You can initialize the UI module at your application's entry point.
pub fn init(cx: &mut gpui::AppContext) {
    theme::init(cx);
    context_menu::init(cx);
    date_picker::init(cx);
    dock::init(cx);
    drawer::init(cx);
    dropdown::init(cx);
    input::init(cx);
    list::init(cx);
    modal::init(cx);
    popover::init(cx);
    popup_menu::init(cx);
    table::init(cx);
    webview::init(cx);
}

pub fn locale() -> impl Deref<Target = str> {
    rust_i18n::locale()
}

pub fn set_locale(locale: &str) {
    rust_i18n::set_locale(locale)
}
