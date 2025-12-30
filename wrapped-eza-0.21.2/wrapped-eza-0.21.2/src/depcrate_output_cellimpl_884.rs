// Generated macro for impl_884 (impl)
macro_rules! Depcrate_output_cellimpl_884 {
() => {
// Module: crate::output::cell
// Provides: {"impl_884"}
// Dependencies: {}
impl TextCellContents { # [doc = " Produces an `ANSIStrings` value that can be used to print the styled"] # [doc = " values of this cell as an ANSI-terminal-formatted string."] pub fn strings (& self) -> ANSIStrings < '_ > { ANSIStrings (& self . 0) } # [doc = " Calculates the width that a cell with these contents would take up, by"] # [doc = " counting the number of characters in each unformatted ANSI string."] pub fn width (& self) -> DisplayWidth { self . 0 . iter () . map (| anstr | DisplayWidth :: from (anstr . as_str ())) . sum () } # [doc = " Promotes these contents to a full cell containing them alongside"] # [doc = " their calculated width."] pub fn promote (self) -> TextCell { TextCell { width : self . width () , contents : self , } } }
};
}
