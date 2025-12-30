// Generated macro for impl_260 (impl)
macro_rules! Depcrate_to_tokensimpl_260 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_260"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for Punct { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend_one (TokenTree :: from (self . clone ())) ; } }
};
}
