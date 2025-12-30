// Generated macro for impl_264 (impl)
macro_rules! Depcrate_to_tokensimpl_264 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_264"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens + ? Sized > ToTokens for Box < T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) } }
};
}
