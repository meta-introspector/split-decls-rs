// Generated macro for impl_37 (impl)
macro_rules! Depcrate_to_tokensimpl_37 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for Rc < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
