// Generated macro for impl_262 (impl)
macro_rules! Depcrate_to_tokensimpl_262 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_262"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens + ? Sized > ToTokens for & T { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) } }
};
}
