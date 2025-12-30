// Generated macro for fn_like_error (function)
macro_rules! Depcratefn_like_error {
() => {
// Module: crate
// Provides: {"fn_like_error"}
// Dependencies: {}
# [proc_macro] pub fn fn_like_error (args : TokenStream) -> TokenStream { format ! ("compile_error!(\"fn_like_error!({})\");" , args) . parse () . unwrap () }
};
}
