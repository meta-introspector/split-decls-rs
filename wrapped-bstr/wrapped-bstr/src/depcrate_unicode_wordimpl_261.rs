// Generated macro for impl_261 (impl)
macro_rules! Depcrate_unicode_wordimpl_261 {
() => {
// Module: crate::unicode::word
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'a > Iterator for WordsWithBreakIndices < 'a > { type Item = (usize , usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , usize , & 'a str) > { let index = self . forward_index ; let (word , size) = decode_word (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; self . forward_index += size ; Some ((index , index + size , word)) } }
};
}
