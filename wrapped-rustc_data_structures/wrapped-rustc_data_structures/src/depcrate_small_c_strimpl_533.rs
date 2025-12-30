// Generated macro for impl_533 (impl)
macro_rules! Depcrate_small_c_strimpl_533 {
() => {
// Module: crate::small_c_str
// Provides: {"impl_533"}
// Dependencies: {}
impl From < & ffi :: CStr > for SmallCStr { fn from (s : & ffi :: CStr) -> Self { Self { data : SmallVec :: from_slice (s . to_bytes_with_nul ()) } } }
};
}
