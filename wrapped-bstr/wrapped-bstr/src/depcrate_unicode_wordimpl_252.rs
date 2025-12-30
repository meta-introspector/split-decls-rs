// Generated macro for impl_252 (impl)
macro_rules! Depcrate_unicode_wordimpl_252 {
() => {
// Module: crate::unicode::word
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'a > Iterator for Words < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { for word in self . 0 . by_ref () { let input = Input :: new (word) . anchored (Anchored :: Yes) . earliest (true) ; if SIMPLE_WORD_FWD . try_search_fwd (& input) . unwrap () . is_some () { return Some (word) ; } } None } }
};
}
