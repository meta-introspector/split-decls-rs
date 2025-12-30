// Generated macro for impl_59 (impl)
macro_rules! Depcrate_styleimpl_59 {
() => {
// Module: crate::style
// Provides: {"impl_59"}
// Dependencies: {}
impl From < (Color , Modifier) > for Style { # [doc = " Creates a new `Style` with the given foreground color and modifier added."] # [doc = ""] # [doc = " To specify multiple modifiers, use the `|` operator."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Color, Modifier, Style};"] # [doc = ""] # [doc = " // red foreground, add bold and italic"] # [doc = " let style = Style::from((Color::Red, Modifier::BOLD | Modifier::ITALIC));"] # [doc = " ```"] fn from ((fg , modifier) : (Color , Modifier)) -> Self { Self :: new () . fg (fg) . add_modifier (modifier) } }
};
}
