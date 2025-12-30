// Generated macro for impl_285 (impl)
macro_rules! Depcrate_to_tokensimpl_285 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_285"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for String { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: string (self) . to_tokens (tokens) } }
};
}
