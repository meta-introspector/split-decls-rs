// Generated macro for impl_20 (impl)
macro_rules! Depcrate_queriesimpl_20 {
() => {
// Module: crate::queries
// Provides: {"impl_20"}
// Dependencies: {}
impl ToTokens for InputQuery { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let fn_ident = & sig . ident ; let create_data_ident = & self . create_data_ident ; let method = quote ! { # sig { let data = # create_data_ident (self) ; data .# fn_ident (self) . unwrap () } } ; method . to_tokens (tokens) ; } }
};
}
