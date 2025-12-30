// Generated macro for async_trait (function)
macro_rules! Depcrateasync_trait {
() => {
// Module: crate
// Provides: {"async_trait"}
// Dependencies: {}
# [proc_macro_attribute] pub fn async_trait (args : TokenStream , input : TokenStream) -> TokenStream { let args = parse_macro_input ! (args as Args) ; let mut item = parse_macro_input ! (input as Item) ; expand (& mut item , args . local) ; TokenStream :: from (quote ! (# item)) }
};
}
