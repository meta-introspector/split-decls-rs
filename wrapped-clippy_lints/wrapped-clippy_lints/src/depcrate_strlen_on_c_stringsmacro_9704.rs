// Generated macro for macro_9704 (macro)
macro_rules! Depcrate_strlen_on_c_stringsmacro_9704 {
() => {
// Module: crate::strlen_on_c_strings
// Provides: {"macro_9704"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `libc::strlen` on a `CString` or `CStr` value,"] # [doc = " and suggest calling `as_bytes().len()` or `to_bytes().len()` respectively instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This avoids calling an unsafe `libc` function."] # [doc = " Currently, it also avoids calculating the length."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust, ignore"] # [doc = " use std::ffi::CString;"] # [doc = " let cstring = CString::new(\"foo\").expect(\"CString::new failed\");"] # [doc = " let len = unsafe { libc::strlen(cstring.as_ptr()) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust, no_run"] # [doc = " use std::ffi::CString;"] # [doc = " let cstring = CString::new(\"foo\").expect(\"CString::new failed\");"] # [doc = " let len = cstring.as_bytes().len();"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub STRLEN_ON_C_STRINGS , complexity , "using `libc::strlen` on a `CString` or `CStr` value, while `as_bytes().len()` or `to_bytes().len()` respectively can be used instead" }
};
}
