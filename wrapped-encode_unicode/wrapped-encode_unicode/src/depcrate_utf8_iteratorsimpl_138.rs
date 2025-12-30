// Generated macro for impl_138 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_138 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'a > Iterator for Utf8Chars < 'a > { type Item = Utf8Char ; fn next (& mut self) -> Option < Utf8Char > { self . 0 . next () . map (| (_ , u8c) | u8c) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
