// Generated macro for varule_derive (function)
macro_rules! Depcratevarule_derive {
() => {
// Module: crate
// Provides: {"varule_derive"}
// Dependencies: {}
# [doc = " Full docs for this proc macro can be found on the [`zerovec`](https://docs.rs/zerovec) crate."] # [proc_macro_derive (VarULE)] pub fn varule_derive (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; TokenStream :: from (varule :: derive_impl (& input , None)) }
};
}
