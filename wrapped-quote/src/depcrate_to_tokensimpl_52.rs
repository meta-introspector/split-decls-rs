// Generated macro for impl_52 (impl)
macro_rules! Depcrate_to_tokensimpl_52 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_52"}
// Dependencies: {}
impl ToTokens for usize { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: usize_suffixed (* self)) ; } }
};
}
