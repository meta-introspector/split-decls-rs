// Generated macro for impl_58 (impl)
macro_rules! Depcrate_styleimpl_58 {
() => {
// Module: crate::style
// Provides: {"impl_58"}
// Dependencies: {}
impl From < (Modifier , Modifier) > for Style { # [doc = " Creates a new `Style` with the given modifiers added and removed."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use ratatui_core::style::{Modifier, Style};"] # [doc = ""] # [doc = " // add bold and italic, remove dim"] # [doc = " let style = Style::from((Modifier::BOLD | Modifier::ITALIC, Modifier::DIM));"] # [doc = " ```"] fn from ((add_modifier , sub_modifier) : (Modifier , Modifier)) -> Self { Self :: new () . add_modifier (add_modifier) . remove_modifier (sub_modifier) } }
};
}
