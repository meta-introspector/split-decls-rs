// Generated macro for impl_255 (impl)
macro_rules! Depcrate_unicode_wordimpl_255 {
() => {
// Module: crate::unicode::word
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a > Iterator for WordIndices < 'a > { type Item = (usize , usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , usize , & 'a str) > { for (start , end , word) in self . 0 . by_ref () { let input = Input :: new (word) . anchored (Anchored :: Yes) . earliest (true) ; if SIMPLE_WORD_FWD . try_search_fwd (& input) . unwrap () . is_some () { return Some ((start , end , word)) ; } } None } }
};
}
