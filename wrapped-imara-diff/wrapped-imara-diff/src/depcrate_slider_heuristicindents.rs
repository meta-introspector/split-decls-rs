// Generated macro for Indents (struct)
macro_rules! Depcrate_slider_heuristicIndents {
() => {
// Module: crate::slider_heuristic
// Provides: {"Indents"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Indents { # [doc = " indent level of the line/token"] indent : IndentLevel , # [doc = " indent level at the previous (non-blank) line"] prev_indent : IndentLevel , # [doc = " indent level at the next (non-blank) line"] next_indent : IndentLevel , # [doc = " How many consecutive lines above the split are blank?"] leading_blanks : u8 , # [doc = " How many lines after the line following the split are blank?"] trailing_blanks : u8 , }
};
}
