// Generated macro for impl_56 (impl)
macro_rules! Depcrate_to_tokensimpl_56 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_56"}
// Dependencies: {}
impl ToTokens for bool { fn to_tokens (& self , tokens : & mut TokenStream) { let word = if * self { "true" } else { "false" } ; tokens . append (Ident :: new (word , Span :: call_site ())) ; } }
};
}
