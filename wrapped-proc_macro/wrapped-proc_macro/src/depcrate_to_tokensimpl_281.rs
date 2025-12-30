// Generated macro for impl_281 (impl)
macro_rules! Depcrate_to_tokensimpl_281 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_281"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for isize { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: isize_suffixed (* self) . to_tokens (tokens) } }
};
}
