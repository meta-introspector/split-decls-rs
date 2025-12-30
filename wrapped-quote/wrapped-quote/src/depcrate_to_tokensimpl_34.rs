// Generated macro for impl_34 (impl)
macro_rules! Depcrate_to_tokensimpl_34 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_34"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > ToTokens for & mut T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
