// Generated macro for CharGrid (struct)
macro_rules! Depcrate_canvasCharGrid {
() => {
// Module: crate::canvas
// Provides: {"CharGrid"}
// Dependencies: {}
# [doc = " The `CharGrid` is a grid made up of cells each containing a single character."] # [doc = ""] # [doc = " This makes it possible to draw shapes with a resolution of 1x1 dots per cell. This is useful"] # [doc = " when you want to draw shapes with a low resolution."] # [derive (Debug)] struct CharGrid { # [doc = " Width of the grid in number of terminal columns"] width : u16 , # [doc = " Height of the grid in number of terminal rows"] height : u16 , # [doc = " Represents a single character for each cell"] cells : Vec < char > , # [doc = " The color of each cell"] colors : Vec < Color > , # [doc = " The character to use for every cell - e.g. a block, dot, etc."] cell_char : char , }
};
}
