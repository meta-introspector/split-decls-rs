// Generated macro for impl_57 (impl)
macro_rules! Depcrate_to_tokensimpl_57 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_57"}
// Dependencies: {}
impl ToTokens for CStr { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: c_string (self)) ; } }
};
}
