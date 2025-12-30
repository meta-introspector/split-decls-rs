// Generated macro for impl_11 (impl)
macro_rules! Depcrate_as_bytesimpl_11 {
() => {
// Module: crate::as_bytes
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : Safe > AsBytes for [T] { # [inline] fn as_bytes (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . as_ptr () as * const u8 , mem :: size_of_val (self)) } } }
};
}
