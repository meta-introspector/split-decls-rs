// Generated macro for impl_358 (impl)
macro_rules! Depcrate_parser_utilsimpl_358 {
() => {
// Module: crate::parser::utils
// Provides: {"impl_358"}
// Dependencies: {}
impl SourcePosition { # [doc (hidden)] pub fn new (index : usize , line : usize , col : usize) -> SourcePosition { assert ! (index >= line + col) ; SourcePosition { index , line , col } } # [doc (hidden)] pub fn new_origin () -> SourcePosition { SourcePosition { index : 0 , line : 0 , col : 0 , } } # [doc (hidden)] pub fn advance_col (& mut self) { self . index += 1 ; self . col += 1 ; } # [doc (hidden)] pub fn advance_line (& mut self) { self . index += 1 ; self . line += 1 ; self . col = 0 ; } # [doc = " The index of the character in the input source"] # [doc = ""] # [doc = " Zero-based index. Take a substring of the original source starting at"] # [doc = " this index to access the item pointed to by this `SourcePosition`."] pub fn index (& self) -> usize { self . index } # [doc = " The line of the character in the input source"] # [doc = ""] # [doc = " Zero-based index: the first line is line zero."] pub fn line (& self) -> usize { self . line } # [doc = " The column of the character in the input source"] # [doc = ""] # [doc = " Zero-based index: the first column is column zero."] pub fn column (& self) -> usize { self . col } }
};
}
