// Generated macro for make_ule (function)
macro_rules! Depcratemake_ule {
() => {
// Module: crate
// Provides: {"make_ule"}
// Dependencies: {}
# [doc = " Full docs for this proc macro can be found on the [`zerovec`](https://docs.rs/zerovec) crate."] # [proc_macro_attribute] pub fn make_ule (attr : TokenStream , item : TokenStream) -> TokenStream { let input = parse_macro_input ! (item as DeriveInput) ; let attr = parse_macro_input ! (attr as Ident) ; TokenStream :: from (make_ule :: make_ule_impl (attr , input)) }
};
}
