// Generated macro for to_compile_error (function)
macro_rules! Depcrateto_compile_error {
() => {
// Module: crate
// Provides: {"to_compile_error"}
// Dependencies: {}
fn to_compile_error (error : syn :: Error , dummy : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let compile_errors = error . to_compile_error () ; quote :: quote ! (# dummy # compile_errors) }
};
}
