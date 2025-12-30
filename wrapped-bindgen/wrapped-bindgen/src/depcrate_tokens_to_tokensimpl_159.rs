// Generated macro for impl_159 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_159 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_159"}
// Dependencies: {}
impl < T : ? Sized + ToOwned + ToTokens > ToTokens for Cow < '_ , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
