// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl ToTokens for private { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . append (Ident :: new (concat ! ("__private" , env ! ("CARGO_PKG_VERSION_PATCH")) , Span :: call_site () ,)) ; } }
};
}
