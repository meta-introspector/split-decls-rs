// Generated macro for BrailleGrid (struct)
macro_rules! Depcrate_canvasBrailleGrid {
() => {
// Module: crate::canvas
// Provides: {"BrailleGrid"}
// Dependencies: {}
# [doc = " The `BrailleGrid` is a grid made up of cells each containing a Braille pattern."] # [doc = ""] # [doc = " This makes it possible to draw shapes with a resolution of 2x4 dots per cell. This is useful"] # [doc = " when you want to draw shapes with a high resolution. Font support for Braille patterns is"] # [doc = " required to see the dots. If your terminal or font does not support this unicode block, you"] # [doc = " will see unicode replacement characters (�) instead of braille dots."] # [doc = ""] # [doc = " This grid type only supports a single foreground color for each 2x4 dots cell. There is no way"] # [doc = " to set the individual color of each dot in the braille pattern."] # [derive (Debug)] struct BrailleGrid { # [doc = " Width of the grid in number of terminal columns"] width : u16 , # [doc = " Height of the grid in number of terminal rows"] height : u16 , # [doc = " Represents the unicode braille patterns. Will take a value between `0x2800` and `0x28FF`"] # [doc = " this is converted to an utf16 string when converting to a layer. See"] # [doc = " <https://en.wikipedia.org/wiki/Braille_Patterns> for more info."] utf16_code_points : Vec < u16 > , # [doc = " The color of each cell only supports foreground colors for now as there's no way to"] # [doc = " individually set the background color of each dot in the braille pattern."] colors : Vec < Color > , }
};
}
