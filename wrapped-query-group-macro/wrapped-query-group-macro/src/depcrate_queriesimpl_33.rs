// Generated macro for impl_33 (impl)
macro_rules! Depcrate_queriesimpl_33 {
() => {
// Module: crate::queries
// Provides: {"impl_33"}
// Dependencies: {}
impl ToTokens for Lookup { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let wrapper_struct = self . interned_struct_path . to_token_stream () ; let method = quote ! { # sig { let zalsa = self . zalsa () ; # wrapper_struct :: ingredient (zalsa) . data (zalsa , id . as_id ()) . 0 . clone () } } ; method . to_tokens (tokens) ; } }
};
}
