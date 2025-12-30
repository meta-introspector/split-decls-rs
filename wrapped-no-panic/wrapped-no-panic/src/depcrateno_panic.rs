// Generated macro for no_panic (function)
macro_rules! Depcrateno_panic {
() => {
// Module: crate
// Provides: {"no_panic"}
// Dependencies: {}
# [proc_macro_attribute] pub fn no_panic (args : TokenStream , input : TokenStream) -> TokenStream { let args = TokenStream2 :: from (args) ; let input = TokenStream2 :: from (input) ; TokenStream :: from (match parse (args , input . clone ()) { Ok (function) => { let expanded = expand_no_panic (function) ; quote ! { # [cfg (not (doc))] # expanded # [cfg (doc)] # input } } Err (parse_error) => { let compile_error = parse_error . to_compile_error () ; quote ! { # compile_error # input } } }) }
};
}
