// Generated macro for impl_45 (impl)
macro_rules! Depcrate_to_tokensimpl_45 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_45"}
// Dependencies: {}
impl ToTokens for i128 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i128_suffixed (* self)) ; } }
};
}
