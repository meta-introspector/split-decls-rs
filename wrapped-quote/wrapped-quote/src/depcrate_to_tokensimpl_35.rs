// Generated macro for impl_35 (impl)
macro_rules! Depcrate_to_tokensimpl_35 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , T : ? Sized + ToOwned + ToTokens > ToTokens for Cow < 'a , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
