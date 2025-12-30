// Generated macro for Style (struct)
macro_rules! Depcrate_styleStyle {
() => {
// Module: crate::style
// Provides: {"Style"}
// Dependencies: {}
# [doc = " A style is a collection of properties that can format a string"] # [doc = " using ANSI escape codes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::{Style, Color};"] # [doc = ""] # [doc = " let style = Style::new().bold().on(Color::Black);"] # [doc = " println!(\"{}\", style.paint(\"Bold on black\"));"] # [doc = " ```"] # [derive (Eq , PartialEq , Clone , Copy)] # [cfg_attr (feature = "derive_serde_style" , derive (serde :: Deserialize , serde :: Serialize))] pub struct Style { # [doc = " The style's foreground color, if it has one."] pub foreground : Option < Color > , # [doc = " The style's background color, if it has one."] pub background : Option < Color > , # [doc = " Whether this style is bold."] pub is_bold : bool , # [doc = " Whether this style is dimmed."] pub is_dimmed : bool , # [doc = " Whether this style is italic."] pub is_italic : bool , # [doc = " Whether this style is underlined."] pub is_underline : bool , # [doc = " Whether this style is blinking."] pub is_blink : bool , # [doc = " Whether this style has reverse colors."] pub is_reverse : bool , # [doc = " Whether this style is hidden."] pub is_hidden : bool , # [doc = " Whether this style is struckthrough."] pub is_strikethrough : bool , # [doc = " Wether this style is always displayed starting with a reset code to clear any remaining style artifacts"] pub prefix_with_reset : bool , }
};
}
