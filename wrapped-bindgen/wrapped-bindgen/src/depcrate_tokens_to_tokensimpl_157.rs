// Generated macro for impl_157 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_157 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for & T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
