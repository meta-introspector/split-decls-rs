// Generated macro for impl_256 (impl)
macro_rules! Depcrate_to_tokensimpl_256 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_256"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for TokenTree { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend_one (self . clone ()) ; } fn into_token_stream (self) -> TokenStream { let mut builder = ConcatTreesHelper :: new (1) ; builder . push (self) ; builder . build () } }
};
}
