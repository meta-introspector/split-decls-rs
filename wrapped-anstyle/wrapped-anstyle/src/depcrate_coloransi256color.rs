// Generated macro for Ansi256Color (struct)
macro_rules! Depcrate_colorAnsi256Color {
() => {
// Module: crate::color
// Provides: {"Ansi256Color"}
// Dependencies: {}
# [doc = " 256 (8-bit) color support"] # [doc = ""] # [doc = " - `0..16` are [`AnsiColor`] palette codes"] # [doc = " - `0..232` map to [`RgbColor`] color values"] # [doc = " - `232..` map to [`RgbColor`] gray-scale values"] # [allow (clippy :: exhaustive_structs)] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct Ansi256Color (pub u8) ;
};
}
