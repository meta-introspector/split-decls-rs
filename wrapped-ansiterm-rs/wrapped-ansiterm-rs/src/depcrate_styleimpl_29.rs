// Generated macro for impl_29 (impl)
macro_rules! Depcrate_styleimpl_29 {
() => {
// Module: crate::style
// Provides: {"impl_29"}
// Dependencies: {}
impl From < Colour > for Style { # [doc = " You can turn a `Colour` into a `Style` with the foreground colour set"] # [doc = " with the `From` trait."] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::{Style, Colour};"] # [doc = " let green_foreground = Style::default().fg(Colour::Green);"] # [doc = " assert_eq!(green_foreground, Colour::Green.normal());"] # [doc = " assert_eq!(green_foreground, Colour::Green.into());"] # [doc = " assert_eq!(green_foreground, Style::from(Colour::Green));"] # [doc = " ```"] fn from (colour : Colour) -> Style { colour . normal () } }
};
}
