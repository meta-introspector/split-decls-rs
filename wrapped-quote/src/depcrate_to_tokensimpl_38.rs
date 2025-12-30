// Generated macro for impl_38 (impl)
macro_rules! Depcrate_to_tokensimpl_38 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (t) = self { t . to_tokens (tokens) ; } } }
};
}
