// Generated macro for impl_53 (impl)
macro_rules! Depcrate_to_tokensimpl_53 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_53"}
// Dependencies: {}
impl ToTokens for f32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: f32_suffixed (* self)) ; } }
};
}
