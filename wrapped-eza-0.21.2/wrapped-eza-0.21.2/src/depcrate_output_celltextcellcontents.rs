// Generated macro for TextCellContents (struct)
macro_rules! Depcrate_output_cellTextCellContents {
() => {
// Module: crate::output::cell
// Provides: {"TextCellContents"}
// Dependencies: {}
# [doc = " The contents of a text cell, as a vector of ANSI-styled strings."] # [doc = ""] # [doc = " It’s possible to use this type directly in the case where you want a"] # [doc = " `TextCell` but aren’t concerned with tracking its width, because it occurs"] # [doc = " in the final cell of a table or grid and there’s no point padding it. This"] # [doc = " happens when dealing with file names."] # [derive (PartialEq , Debug , Clone , Default)] pub struct TextCellContents (Vec < ANSIString < 'static > >) ;
};
}
