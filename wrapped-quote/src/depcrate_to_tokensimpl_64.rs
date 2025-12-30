// Generated macro for impl_64 (impl)
macro_rules! Depcrate_to_tokensimpl_64 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_64"}
// Dependencies: {}
impl ToTokens for TokenStream { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (iter :: once (self . clone ())) ; } fn into_token_stream (self) -> TokenStream { self } }
};
}
