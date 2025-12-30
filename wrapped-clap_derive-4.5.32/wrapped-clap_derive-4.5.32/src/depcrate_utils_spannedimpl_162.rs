// Generated macro for impl_162 (impl)
macro_rules! Depcrate_utils_spannedimpl_162 {
() => {
// Module: crate::utils::spanned
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Sp < T > { fn to_tokens (& self , stream : & mut TokenStream) { let tt = self . val . to_token_stream () . into_iter () . map (| mut tt | { tt . set_span (self . span) ; tt }) ; stream . extend (tt) ; } }
};
}
