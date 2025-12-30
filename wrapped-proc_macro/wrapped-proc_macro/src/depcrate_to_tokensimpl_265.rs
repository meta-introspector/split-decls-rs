// Generated macro for impl_265 (impl)
macro_rules! Depcrate_to_tokensimpl_265 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_265"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens + ? Sized > ToTokens for Rc < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) } }
};
}
