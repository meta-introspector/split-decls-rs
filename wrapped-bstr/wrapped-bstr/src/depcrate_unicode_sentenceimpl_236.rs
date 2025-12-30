// Generated macro for impl_236 (impl)
macro_rules! Depcrate_unicode_sentenceimpl_236 {
() => {
// Module: crate::unicode::sentence
// Provides: {"impl_236"}
// Dependencies: {}
impl < 'a > Iterator for Sentences < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (sentence , size) = decode_sentence (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (sentence) } }
};
}
