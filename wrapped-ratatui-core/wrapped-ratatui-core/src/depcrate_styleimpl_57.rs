// Generated macro for impl_57 (impl)
macro_rules! Depcrate_styleimpl_57 {
() => {
// Module: crate::style
// Provides: {"impl_57"}
// Dependencies: {}
impl From < Modifier > for Style { # [doc = " Creates a new `Style` with the given modifier added."] # [doc = ""] # [doc = " To specify multiple modifiers, use the `|` operator."] # [doc = ""] # [doc = " To specify modifiers to add and remove, use the `from((add_modifier, sub_modifier))`"] # [doc = " constructor."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Style, Modifier};"] # [doc = ""] # [doc = " // add bold and italic"] # [doc = " let style = Style::from(Modifier::BOLD|Modifier::ITALIC);"] fn from (modifier : Modifier) -> Self { Self :: new () . add_modifier (modifier) } }
};
}
