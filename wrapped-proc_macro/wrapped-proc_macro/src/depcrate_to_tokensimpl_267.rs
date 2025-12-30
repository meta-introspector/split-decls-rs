// Generated macro for impl_267 (impl)
macro_rules! Depcrate_to_tokensimpl_267 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_267"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens > ToTokens for Option < T > { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (t) = self { t . to_tokens (tokens) ; } } }
};
}
