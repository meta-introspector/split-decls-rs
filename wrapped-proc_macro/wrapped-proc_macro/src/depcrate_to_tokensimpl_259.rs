// Generated macro for impl_259 (impl)
macro_rules! Depcrate_to_tokensimpl_259 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_259"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for Ident { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend_one (TokenTree :: from (self . clone ())) ; } }
};
}
