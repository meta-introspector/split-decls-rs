// Generated macro for make_varule (function)
macro_rules! Depcratemake_varule {
() => {
// Module: crate
// Provides: {"make_varule"}
// Dependencies: {}
# [doc = " Full docs for this proc macro can be found on the [`zerovec`](https://docs.rs/zerovec) crate."] # [proc_macro_attribute] pub fn make_varule (attr : TokenStream , item : TokenStream) -> TokenStream { let input = parse_macro_input ! (item as DeriveInput) ; let attr = parse_macro_input ! (attr as Ident) ; TokenStream :: from (make_varule :: make_varule_impl (attr , input)) }
};
}
