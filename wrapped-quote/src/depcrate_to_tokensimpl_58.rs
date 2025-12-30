// Generated macro for impl_58 (impl)
macro_rules! Depcrate_to_tokensimpl_58 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_58"}
// Dependencies: {}
impl ToTokens for CString { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: c_string (self)) ; } }
};
}
