// Generated macro for impl_263 (impl)
macro_rules! Depcrate_to_tokensimpl_263 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_263"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens + ? Sized > ToTokens for & mut T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) } }
};
}
