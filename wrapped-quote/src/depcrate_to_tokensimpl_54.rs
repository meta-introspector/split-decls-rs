// Generated macro for impl_54 (impl)
macro_rules! Depcrate_to_tokensimpl_54 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_54"}
// Dependencies: {}
impl ToTokens for f64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: f64_suffixed (* self)) ; } }
};
}
