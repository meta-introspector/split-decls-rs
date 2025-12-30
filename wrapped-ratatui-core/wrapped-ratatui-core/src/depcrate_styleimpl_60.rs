// Generated macro for impl_60 (impl)
macro_rules! Depcrate_styleimpl_60 {
() => {
// Module: crate::style
// Provides: {"impl_60"}
// Dependencies: {}
impl From < (Color , Color , Modifier) > for Style { # [doc = " Creates a new `Style` with the given foreground and background colors and modifier added."] # [doc = ""] # [doc = " To specify multiple modifiers, use the `|` operator."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Color, Modifier, Style};"] # [doc = ""] # [doc = " // red foreground, blue background, add bold and italic"] # [doc = " let style = Style::from((Color::Red, Color::Blue, Modifier::BOLD | Modifier::ITALIC));"] # [doc = " ```"] fn from ((fg , bg , modifier) : (Color , Color , Modifier)) -> Self { Self :: new () . fg (fg) . bg (bg) . add_modifier (modifier) } }
};
}
