// Generated macro for split (function)
macro_rules! Depcrate_boundarysplit {
() => {
// Module: crate::boundary
// Provides: {"split"}
// Dependencies: {}
# [doc = " Split an identifier into a list of words using the list of boundaries."] # [doc = ""] # [doc = " This is used internally for splitting an identifier before mutating by"] # [doc = " a pattern and joining again with a delimiter."] # [doc = " ```"] # [doc = " use convert_case::{Boundary, split};"] # [doc = " assert_eq!("] # [doc = "     split(&\"one_two-three.four\", &[Boundary::Underscore, Boundary::Hyphen]),"] # [doc = "     vec![\"one\", \"two\", \"three.four\"],"] # [doc = " )"] # [doc = " ```"] pub fn split < 's , T > (s : & 's T , boundaries : & [Boundary]) -> Vec < & 's str > where T : AsRef < str > , { let s = s . as_ref () ; if s . is_empty () { return Vec :: new () ; } let mut words = Vec :: new () ; let mut last_boundary_end = 0 ; let (indices , graphemes) : (Vec < _ > , Vec < _ >) = s . grapheme_indices (true) . unzip () ; let grapheme_length = indices [graphemes . len () - 1] + graphemes [graphemes . len () - 1] . len () ; for i in 0 .. graphemes . len () { for boundary in boundaries { if boundary . matches (& graphemes [i ..]) { let boundary_byte_start : usize = * indices . get (i + boundary . start ()) . unwrap_or (& grapheme_length) ; let boundary_byte_end : usize = * indices . get (i + boundary . start () + boundary . len ()) . unwrap_or (& grapheme_length) ; words . push (& s [last_boundary_end .. boundary_byte_start]) ; last_boundary_end = boundary_byte_end ; break ; } } } words . push (& s [last_boundary_end ..]) ; words . into_iter () . collect () }
};
}
