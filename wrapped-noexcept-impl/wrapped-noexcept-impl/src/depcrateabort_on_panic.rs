// Generated macro for abort_on_panic (function)
macro_rules! Depcrateabort_on_panic {
() => {
// Module: crate
// Provides: {"abort_on_panic"}
// Dependencies: {}
# [proc_macro_attribute] pub fn abort_on_panic (args : TokenStream , input : TokenStream) -> TokenStream { let args = TokenStream2 :: from (args) ; let input = TokenStream2 :: from (input) ; let expanded = match parse (args , input . clone ()) { Ok (function) => expand_abort_on_panic (function) , Err (parse_error) => { let compile_error = parse_error . to_compile_error () ; quote ! (# compile_error # input) } } ; TokenStream :: from (expanded) }
};
}
