// Generated macro for impl_161 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_161 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_161"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for Rc < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
