// Generated macro for impl_13 (impl)
macro_rules! Depcrate_common_defaultimpl_13 {
() => {
// Module: crate::common::default
// Provides: {"impl_13"}
// Dependencies: {}
impl ToTokens for Value { fn to_tokens (& self , into : & mut TokenStream) { match self { Self :: Default => quote ! { :: std :: default :: Default :: default () } , Self :: Expr (expr) => quote ! { (# expr) . into () } , } . to_tokens (into) } }
};
}
