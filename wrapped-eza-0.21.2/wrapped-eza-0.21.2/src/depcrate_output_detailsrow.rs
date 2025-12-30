// Generated macro for Row (struct)
macro_rules! Depcrate_output_detailsRow {
() => {
// Module: crate::output::details
// Provides: {"Row"}
// Dependencies: {}
pub struct Row { # [doc = " Vector of cells to display."] # [doc = ""] # [doc = " Most of the rows will be used to display files’ metadata, so this will"] # [doc = " almost always be `Some`, containing a vector of cells. It will only be"] # [doc = " `None` for a row displaying an attribute or error, neither of which"] # [doc = " have cells."] pub cells : Option < TableRow > , # [doc = " This file’s name, in coloured output. The name is treated separately"] # [doc = " from the other cells, as it never requires padding."] pub name : TextCell , # [doc = " Information used to determine which symbols to display in a tree."] pub tree : TreeParams , }
};
}
