// Generated macro for impl_160 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_160 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for Box < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
