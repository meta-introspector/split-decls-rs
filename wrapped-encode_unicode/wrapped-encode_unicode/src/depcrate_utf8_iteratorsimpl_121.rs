// Generated macro for impl_121 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_121 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_121"}
// Dependencies: {}
impl ExactSizeIterator for Utf8Iterator { fn len (& self) -> usize { let unused_bytes = self . 0 . not () . leading_zeros () / 8 ; 4 - unused_bytes as usize } }
};
}
