// Generated macro for code_to_str (function)
macro_rules! Depcrate_errorcode_to_str {
() => {
// Module: crate::error
// Provides: {"code_to_str"}
// Dependencies: {}
# [must_use] pub fn code_to_str (code : c_int) -> & 'static str { let err_str = unsafe { super :: sqlite3_errstr (code) } ; if err_str . is_null () { "Unknown errod code" } else { unsafe { CStr :: from_ptr (err_str) } . to_str () . unwrap () } }
};
}
