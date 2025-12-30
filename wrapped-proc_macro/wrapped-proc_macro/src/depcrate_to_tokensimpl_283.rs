// Generated macro for impl_283 (impl)
macro_rules! Depcrate_to_tokensimpl_283 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_283"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for char { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: character (* self) . to_tokens (tokens) } }
};
}
