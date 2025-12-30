// Generated macro for impl_46 (impl)
macro_rules! Depcrate_to_tokensimpl_46 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_46"}
// Dependencies: {}
impl ToTokens for isize { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: isize_suffixed (* self)) ; } }
};
}
