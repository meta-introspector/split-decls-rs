// Generated macro for style (function)
macro_rules! Depcrate_stylestyle {
() => {
// Module: crate::style
// Provides: {"style"}
// Dependencies: {}
# [doc = " Creates a `StyledContent`."] # [doc = ""] # [doc = " This could be used to style any type that implements `Display` with colors and text attributes."] # [doc = ""] # [doc = " See [`StyledContent`](struct.StyledContent.html) for more info."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use crossterm::style::{style, Stylize, Color};"] # [doc = ""] # [doc = " let styled_content = style(\"Blue colored text on yellow background\")"] # [doc = "     .with(Color::Blue)"] # [doc = "     .on(Color::Yellow);"] # [doc = ""] # [doc = " println!(\"{}\", styled_content);"] # [doc = " ```"] pub fn style < D : Display > (val : D) -> StyledContent < D > { ContentStyle :: new () . apply (val) }
};
}
