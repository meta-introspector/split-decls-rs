// Generated macro for impl_118 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_118 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_118"}
// Dependencies: {}
impl From < Utf8Char > for Utf8Iterator { fn from (uc : Utf8Char) -> Self { let used = u32 :: from_le_bytes (uc . to_array () . 0) ; let unused_set = (u64 :: MAX << (uc . len () as u64 * 8)) as u32 ; Utf8Iterator (used | unused_set) } }
};
}
