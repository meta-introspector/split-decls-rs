// Generated macro for impl_221 (impl)
macro_rules! Depcrate_unicode_graphemeimpl_221 {
() => {
// Module: crate::unicode::grapheme
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > Iterator for Graphemes < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (grapheme , size) = decode_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (grapheme) } }
};
}
