// Generated macro for error_message (function)
macro_rules! Depcrate_testerror_message {
() => {
// Module: crate::test
// Provides: {"error_message"}
// Dependencies: {}
pub fn error_message (ptr : * const c_char) -> String { let c_str = unsafe { CStr :: from_ptr (ptr as * const _) } ; let s = str :: from_utf8 (c_str . to_bytes ()) . unwrap () . to_owned () ; unsafe { free (ptr as * mut c_void) ; } s }
};
}
