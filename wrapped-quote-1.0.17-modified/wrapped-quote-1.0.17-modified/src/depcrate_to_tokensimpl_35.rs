// Generated macro for impl_35 (impl)
macro_rules! Depcrate_to_tokensimpl_35 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_35"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for Rc < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
