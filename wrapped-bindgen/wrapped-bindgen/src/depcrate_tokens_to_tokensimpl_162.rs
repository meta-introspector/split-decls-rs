// Generated macro for impl_162 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_162 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (ref t) = * self { t . to_tokens (tokens) ; } } }
};
}
