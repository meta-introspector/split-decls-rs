// Generated macro for error_message (function)
macro_rules! Depcrate_ffi_utilerror_message {
() => {
// Module: crate::ffi_util
// Provides: {"error_message"}
// Dependencies: {}
pub fn error_message (ptr : * const c_char) -> String { unsafe { let s = from_cstr (ptr) ; ffi :: rocksdb_free (ptr as * mut c_void) ; s } }
};
}
