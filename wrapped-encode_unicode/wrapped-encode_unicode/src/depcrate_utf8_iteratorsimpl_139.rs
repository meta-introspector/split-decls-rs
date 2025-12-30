// Generated macro for impl_139 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_139 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf8Chars < 'a > { fn next_back (& mut self) -> Option < Utf8Char > { self . 0 . next_back () . map (| (_ , u8c) | u8c) } }
};
}
