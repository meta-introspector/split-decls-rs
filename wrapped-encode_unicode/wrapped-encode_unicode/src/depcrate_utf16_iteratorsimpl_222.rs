// Generated macro for impl_222 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_222 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a > Iterator for Utf16Chars < 'a > { type Item = Utf16Char ; fn next (& mut self) -> Option < Utf16Char > { self . 0 . next () . map (| (_ , u16c) | u16c) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
