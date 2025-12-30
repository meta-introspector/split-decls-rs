// Generated macro for macro_7085 (macro)
macro_rules! Depcrate_methodsmacro_7085 {
() => {
// Module: crate::methods
// Provides: {"macro_7085"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the manual creation of C strings (a string with a `NUL` byte at the end), either"] # [doc = " through one of the `CStr` constructor functions, or more plainly by calling `.as_ptr()`"] # [doc = " on a (byte) string literal with a hardcoded `\\0` byte at the end."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely using `c\"str\"` literals and is also less error-prone,"] # [doc = " because the compiler checks for interior `NUL` bytes and the terminating `NUL` byte is inserted automatically."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::ffi::CStr;"] # [doc = " # mod libc { pub unsafe fn puts(_: *const i8) {} }"] # [doc = " fn needs_cstr(_: &CStr) {}"] # [doc = ""] # [doc = " needs_cstr(CStr::from_bytes_with_nul(b\"Hello\\0\").unwrap());"] # [doc = " unsafe { libc::puts(\"World\\0\".as_ptr().cast()) }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::ffi::CStr;"] # [doc = " # mod libc { pub unsafe fn puts(_: *const i8) {} }"] # [doc = " fn needs_cstr(_: &CStr) {}"] # [doc = ""] # [doc = " needs_cstr(c\"Hello\");"] # [doc = " unsafe { libc::puts(c\"World\".as_ptr()) }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub MANUAL_C_STR_LITERALS , complexity , r#"creating a `CStr` through functions when `c""` literals can be used"# }
};
}
