// Generated macro for impl_20 (impl)
macro_rules! Depcrate_boundimpl_20 {
() => {
// Module: crate::bound
// Provides: {"impl_20"}
// Dependencies: {}
impl ToTokens for InferredBound { fn to_tokens (& self , tokens : & mut TokenStream) { let ident = Ident :: new (self . as_str () , Span :: call_site ()) ; quote ! (:: core :: marker ::# ident) . to_tokens (tokens) ; } }
};
}
