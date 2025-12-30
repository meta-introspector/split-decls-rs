// Generated macro for tests (function)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [proc_macro_attribute] pub fn tests (args : TokenStream , input : TokenStream) -> TokenStream { match tests_impl (args , input) { Ok (ts) => ts , Err (e) => e . to_compile_error () . into () , } }
};
}
