// Generated macro for impl_207 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_207 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_207"}
// Dependencies: {}
impl ExactSizeIterator for Utf16Iterator { fn len (& self) -> usize { (if self . first == FIRST_USED { 0 } else { 1 }) + (if self . second == SECOND_USED { 0 } else { 1 }) } }
};
}
