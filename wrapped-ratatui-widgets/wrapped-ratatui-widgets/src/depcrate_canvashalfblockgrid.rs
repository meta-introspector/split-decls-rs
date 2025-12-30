// Generated macro for HalfBlockGrid (struct)
macro_rules! Depcrate_canvasHalfBlockGrid {
() => {
// Module: crate::canvas
// Provides: {"HalfBlockGrid"}
// Dependencies: {}
# [doc = " The `HalfBlockGrid` is a grid made up of cells each containing a half block character."] # [doc = ""] # [doc = " In terminals, each character is usually twice as tall as it is wide. Unicode has a couple of"] # [doc = " vertical half block characters, the upper half block '▀' and lower half block '▄' which take up"] # [doc = " half the height of a normal character but the full width. Together with an empty space ' ' and a"] # [doc = " full block '█', we can effectively double the resolution of a single cell. In addition, because"] # [doc = " each character can have a foreground and background color, we can control the color of the upper"] # [doc = " and lower half of each cell. This allows us to draw shapes with a resolution of 1x2 \"pixels\" per"] # [doc = " cell."] # [doc = ""] # [doc = " This allows for more flexibility than the `BrailleGrid` which only supports a single"] # [doc = " foreground color for each 2x4 dots cell, and the `CharGrid` which only supports a single"] # [doc = " character for each cell."] # [derive (Debug)] struct HalfBlockGrid { # [doc = " Width of the grid in number of terminal columns"] width : u16 , # [doc = " Height of the grid in number of terminal rows"] height : u16 , # [doc = " Represents a single color for each \"pixel\" arranged in column, row order"] pixels : Vec < Vec < Color > > , }
};
}
