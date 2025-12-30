// Generated macro for impl_104 (impl)
macro_rules! Depcrate_ffi_cimpl_104 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_104"}
// Dependencies: {}
impl Drop for StreamWrapper { fn drop (& mut self) { drop (unsafe { Box :: from_raw (self . inner) }) ; } }
};
}
