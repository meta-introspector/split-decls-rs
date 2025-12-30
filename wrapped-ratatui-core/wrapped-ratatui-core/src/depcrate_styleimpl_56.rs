// Generated macro for impl_56 (impl)
macro_rules! Depcrate_styleimpl_56 {
() => {
// Module: crate::style
// Provides: {"impl_56"}
// Dependencies: {}
impl From < (Color , Color) > for Style { # [doc = " Creates a new `Style` with the given foreground and background colors."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Color, Style};"] # [doc = ""] # [doc = " // red foreground, blue background"] # [doc = " let style = Style::from((Color::Red, Color::Blue));"] # [doc = " // default foreground, blue background"] # [doc = " let style = Style::from((Color::Reset, Color::Blue));"] # [doc = " ```"] fn from ((fg , bg) : (Color , Color)) -> Self { Self :: new () . fg (fg) . bg (bg) } }
};
}
