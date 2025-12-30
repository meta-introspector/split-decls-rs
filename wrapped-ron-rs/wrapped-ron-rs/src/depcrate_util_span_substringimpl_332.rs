// Generated macro for impl_332 (impl)
macro_rules! Depcrate_util_span_substringimpl_332 {
() => {
// Module: crate::util::span_substring
// Provides: {"impl_332"}
// Dependencies: {}
impl Position { # [doc = " Given a Position and a string, return the 0-indexed grapheme index into the"] # [doc = " string at that position, or [None] if the Position is out of bounds of the string."] # [must_use] pub fn grapheme_index (& self , s : & str) -> Option < usize > { use unicode_segmentation :: UnicodeSegmentation ; let mut line_no = 1 ; let mut col_no = 1 ; if (self . line , self . col) == (1 , 1) { return Some (0) ; } let mut i = 0 ; if (line_no , col_no) == (self . line , self . col) { return Some (i) ; } for ch in s . graphemes (true) { if (line_no , col_no) == (self . line , self . col) { return Some (i) ; } if matches ! (ch , "\n" | "\r\n") { line_no += 1 ; col_no = 1 ; } else { col_no += 1 ; } i += 1 ; } if (line_no , col_no) == (self . line , self . col) { return Some (i) ; } None } }
};
}
