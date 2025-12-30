// Generated macro for impl_257 (impl)
macro_rules! Depcrate_to_tokensimpl_257 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_257"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for TokenStream { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (self . clone ()) ; } fn into_token_stream (self) -> TokenStream { self } }
};
}
