// Generated macro for impl_32 (impl)
macro_rules! Depcrate_to_tokensimpl_32 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , T : ? Sized + ToTokens > ToTokens for & 'a mut T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
