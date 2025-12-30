// Generated macro for impl_284 (impl)
macro_rules! Depcrate_to_tokensimpl_284 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_284"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: string (self) . to_tokens (tokens) } }
};
}
