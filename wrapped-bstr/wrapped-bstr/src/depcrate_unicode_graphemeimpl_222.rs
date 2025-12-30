// Generated macro for impl_222 (impl)
macro_rules! Depcrate_unicode_graphemeimpl_222 {
() => {
// Module: crate::unicode::grapheme
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Graphemes < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { let (grapheme , size) = decode_last_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; Some (grapheme) } }
};
}
