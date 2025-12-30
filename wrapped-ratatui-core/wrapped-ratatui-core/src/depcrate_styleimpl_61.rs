// Generated macro for impl_61 (impl)
macro_rules! Depcrate_styleimpl_61 {
() => {
// Module: crate::style
// Provides: {"impl_61"}
// Dependencies: {}
impl From < (Color , Color , Modifier , Modifier) > for Style { # [doc = " Creates a new `Style` with the given foreground and background colors and modifiers added"] # [doc = " and removed."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Color, Modifier, Style};"] # [doc = ""] # [doc = " // red foreground, blue background, add bold and italic, remove dim"] # [doc = " let style = Style::from(("] # [doc = "     Color::Red,"] # [doc = "     Color::Blue,"] # [doc = "     Modifier::BOLD | Modifier::ITALIC,"] # [doc = "     Modifier::DIM,"] # [doc = " ));"] # [doc = " ```"] fn from ((fg , bg , add_modifier , sub_modifier) : (Color , Color , Modifier , Modifier)) -> Self { Self :: new () . fg (fg) . bg (bg) . add_modifier (add_modifier) . remove_modifier (sub_modifier) } }
};
}
