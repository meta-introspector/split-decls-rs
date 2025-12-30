// Generated macro for Style (struct)
macro_rules! Depcrate_styleStyle {
() => {
// Module: crate::style
// Provides: {"Style"}
// Dependencies: {}
# [doc = " A style is a collection of properties that can format a string"] # [doc = " using ANSI escape codes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::{Style, Colour};"] # [doc = ""] # [doc = " let style = Style::new().bold().on(Colour::Black);"] # [doc = " println!(\"{}\", style.paint(\"Bold on black\"));"] # [doc = " ```"] # [derive (PartialEq , Clone , Copy)] # [cfg_attr (feature = "derive_serde_style" , derive (serde :: Deserialize , serde :: Serialize))] pub struct Style { # [doc = " The style's foreground colour, if it has one."] pub foreground : Option < Colour > , # [doc = " The style's background colour, if it has one."] pub background : Option < Colour > , # [doc = " Whether this style is bold."] pub is_bold : bool , # [doc = " Whether this style is dimmed."] pub is_dimmed : bool , # [doc = " Whether this style is italic."] pub is_italic : bool , # [doc = " Whether this style is underlined."] pub is_underline : bool , # [doc = " Whether this style is blinking."] pub is_blink : bool , # [doc = " Whether this style has reverse colours."] pub is_reverse : bool , # [doc = " Whether this style is hidden."] pub is_hidden : bool , # [doc = " Whether this style is struckthrough."] pub is_strikethrough : bool }
};
}
