// Generated macro for impl_113 (impl)
macro_rules! Depcrate_ffi_cimpl_113 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_113"}
// Dependencies: {}
impl < D : Direction > Drop for Stream < D > { fn drop (& mut self) { unsafe { let _ = D :: destroy (self . stream_wrapper . inner) ; } } }
};
}
