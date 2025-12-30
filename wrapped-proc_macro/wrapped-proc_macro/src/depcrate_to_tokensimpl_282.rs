// Generated macro for impl_282 (impl)
macro_rules! Depcrate_to_tokensimpl_282 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_282"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for bool { fn to_tokens (& self , tokens : & mut TokenStream) { let word = if * self { "true" } else { "false" } ; Ident :: new (word , Span :: call_site ()) . to_tokens (tokens) } }
};
}
