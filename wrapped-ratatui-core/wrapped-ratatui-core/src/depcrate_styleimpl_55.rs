// Generated macro for impl_55 (impl)
macro_rules! Depcrate_styleimpl_55 {
() => {
// Module: crate::style
// Provides: {"impl_55"}
// Dependencies: {}
impl From < Color > for Style { # [doc = " Creates a new `Style` with the given foreground color."] # [doc = ""] # [doc = " To specify a foreground and background color, use the `from((fg, bg))` constructor."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Color, Style};"] # [doc = ""] # [doc = " let style = Style::from(Color::Red);"] # [doc = " ```"] fn from (color : Color) -> Self { Self :: new () . fg (color) } }
};
}
