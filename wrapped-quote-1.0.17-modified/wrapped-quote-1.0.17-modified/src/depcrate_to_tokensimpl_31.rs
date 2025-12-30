// Generated macro for impl_31 (impl)
macro_rules! Depcrate_to_tokensimpl_31 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , T : ? Sized + ToTokens > ToTokens for & 'a T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
