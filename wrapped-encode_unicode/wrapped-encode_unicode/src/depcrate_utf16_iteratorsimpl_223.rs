// Generated macro for impl_223 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_223 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf16Chars < 'a > { fn next_back (& mut self) -> Option < Utf16Char > { self . 0 . next_back () . map (| (_ , u16c) | u16c) } }
};
}
