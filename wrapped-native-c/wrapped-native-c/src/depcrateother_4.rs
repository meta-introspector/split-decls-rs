// Generated macro for other_4 (other)
macro_rules! Depcrateother_4 {
() => {
// Module: crate
// Provides: {"other_4"}
// Dependencies: {}
extern "C" { # [doc = " call rust_demangle_demangle"] pub fn rust_demangle_demangle (s : * const c_char , res : * mut CDemangle) ; # [doc = " call rust_demangle_display_demangle"] pub fn rust_demangle_display_demangle (res : * const CDemangle , out : * mut c_char , len : usize , alternate : bool ,) -> c_int ; }
};
}
