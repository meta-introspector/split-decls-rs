// Generated macro for impl_36 (impl)
macro_rules! Depcrate_to_tokensimpl_36 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (ref t) = * self { t . to_tokens (tokens) ; } } }
};
}
