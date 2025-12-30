// Generated macro for macro_437 (macro)
macro_rules! Depcrate_errormacro_437 {
() => {
// Module: crate::error
// Provides: {"macro_437"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { use std :: ffi :: { CString } ; use ffi :: ERR_get_error_all ; type RetStr <'a > = &'a str ; # [derive (Clone)] struct ShimStr (CString) ; impl ShimStr { unsafe fn new (s : * const c_char) -> Self { ShimStr (CStr :: from_ptr (s) . to_owned ()) } fn as_ptr (& self) -> * const c_char { self . 0 . as_ptr () } fn as_str (& self) -> & str { self . 0 . to_str () . unwrap () } } } else { # [allow (bad_style)] unsafe extern "C" fn ERR_get_error_all (file : * mut * const c_char , line : * mut c_int , func : * mut * const c_char , data : * mut * const c_char , flags : * mut c_int ,) -> ErrType { let code = ffi :: ERR_get_error_line_data (file , line , data , flags) ; * func = ffi :: ERR_func_error_string (code) ; code } type RetStr <'a > = &'static str ; # [derive (Clone)] struct ShimStr (* const c_char) ; impl ShimStr { unsafe fn new (s : * const c_char) -> Self { ShimStr (s) } fn as_ptr (& self) -> * const c_char { self . 0 } fn as_str (& self) -> &'static str { unsafe { CStr :: from_ptr (self . 0) . to_str () . unwrap () } } } } }
};
}
