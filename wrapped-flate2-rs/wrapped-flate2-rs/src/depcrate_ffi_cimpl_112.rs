// Generated macro for impl_112 (impl)
macro_rules! Depcrate_ffi_cimpl_112 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_112"}
// Dependencies: {}
impl < D : Direction > Stream < D > { pub fn msg (& self) -> ErrorMessage { let msg = unsafe { (* self . stream_wrapper . inner) . msg } ; ErrorMessage (if msg . is_null () { None } else { let s = unsafe { std :: ffi :: CStr :: from_ptr (msg) } ; std :: str :: from_utf8 (s . to_bytes ()) . ok () }) } }
};
}
