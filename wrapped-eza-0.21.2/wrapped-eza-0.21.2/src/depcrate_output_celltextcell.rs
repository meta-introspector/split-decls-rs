// Generated macro for TextCell (struct)
macro_rules! Depcrate_output_cellTextCell {
() => {
// Module: crate::output::cell
// Provides: {"TextCell"}
// Dependencies: {}
# [doc = " An individual cell that holds text in a table, used in the details and"] # [doc = " lines views to store ANSI-terminal-formatted data before it is printed."] # [doc = ""] # [doc = " A text cell is made up of zero or more strings coupled with the"] # [doc = " pre-computed length of all the strings combined. When constructing details"] # [doc = " or grid-details tables, the length will have to be queried multiple times,"] # [doc = " so it makes sense to cache it."] # [doc = ""] # [doc = " (This used to be called `Cell`, but was renamed because there’s a Rust"] # [doc = " type by that name too.)"] # [derive (PartialEq , Debug , Clone , Default)] pub struct TextCell { # [doc = " The contents of this cell, as a vector of ANSI-styled strings."] pub contents : TextCellContents , # [doc = " The Unicode “display width” of this cell."] pub width : DisplayWidth , }
};
}
