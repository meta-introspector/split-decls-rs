// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl ToTokens for private { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . append (Ident :: new (concat ! ("__private" , env ! ("CARGO_PKG_VERSION_PATCH")) , Span :: call_site () ,)) ; } }
};
}
