// Generated macro for from_cstr (function)
macro_rules! Depcrate_ffi_utilfrom_cstr {
() => {
// Module: crate::ffi_util
// Provides: {"from_cstr"}
// Dependencies: {}
pub (crate) unsafe fn from_cstr (ptr : * const c_char) -> String { let cstr = unsafe { CStr :: from_ptr (ptr as * const _) } ; String :: from_utf8_lossy (cstr . to_bytes ()) . into_owned () }
};
}
