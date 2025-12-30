// Generated macro for IndentStyle (enum)
macro_rules! Depcrate_ppIndentStyle {
() => {
// Module: crate::pp
// Provides: {"IndentStyle"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq)] enum IndentStyle { # [doc = " Vertically aligned under whatever column this block begins at."] # [doc = ""] # [doc = "     fn demo(arg1: usize,"] # [doc = "             arg2: usize) {}"] Visual , # [doc = " Indented relative to the indentation level of the previous line."] # [doc = ""] # [doc = "     fn demo("] # [doc = "         arg1: usize,"] # [doc = "         arg2: usize,"] # [doc = "     ) {}"] Block { offset : isize } , }
};
}
