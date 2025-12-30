// Generated macro for impl_258 (impl)
macro_rules! Depcrate_unicode_wordimpl_258 {
() => {
// Module: crate::unicode::word
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a > Iterator for WordsWithBreaks < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (word , size) = decode_word (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (word) } }
};
}
