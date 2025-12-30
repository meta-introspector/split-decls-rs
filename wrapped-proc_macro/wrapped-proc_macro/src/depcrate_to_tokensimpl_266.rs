// Generated macro for impl_266 (impl)
macro_rules! Depcrate_to_tokensimpl_266 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_266"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl < T : ToTokens + ToOwned + ? Sized > ToTokens for Cow < '_ , T > { fn to_tokens (& self , tokens : & mut TokenStream) { (* * self) . to_tokens (tokens) } }
};
}
