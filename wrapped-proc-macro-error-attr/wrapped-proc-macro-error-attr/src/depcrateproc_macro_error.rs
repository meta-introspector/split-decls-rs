// Generated macro for proc_macro_error (function)
macro_rules! Depcrateproc_macro_error {
() => {
// Module: crate
// Provides: {"proc_macro_error"}
// Dependencies: {}
# [proc_macro_attribute] pub fn proc_macro_error (attr : TokenStream , input : TokenStream) -> TokenStream { match impl_proc_macro_error (attr . into () , input . clone () . into ()) { Ok (ts) => ts , Err (e) => { let error = e . into_compile_error () ; let input = TokenStream2 :: from (input) ; quote ! (# input # error) . into () } } }
};
}
