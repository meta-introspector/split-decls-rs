// Generated macro for impl_33 (impl)
macro_rules! Depcrate_to_tokensimpl_33 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , T : ? Sized + ToOwned + ToTokens > ToTokens for Cow < 'a , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) ; } }
};
}
