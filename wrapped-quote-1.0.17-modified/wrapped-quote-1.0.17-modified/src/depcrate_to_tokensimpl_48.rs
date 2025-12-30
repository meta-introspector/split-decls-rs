// Generated macro for impl_48 (impl)
macro_rules! Depcrate_to_tokensimpl_48 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_48"}
// Dependencies: {}
impl ToTokens for TokenStream { fn to_tokens (& self , dst : & mut TokenStream) { dst . extend (iter :: once (self . clone ())) ; } fn into_token_stream (self) -> TokenStream { self } }
};
}
