// Generated macro for impl_88 (impl)
macro_rules! Depcrate_verbatimimpl_88 {
() => {
// Module: crate::verbatim
// Provides: {"impl_88"}
// Dependencies: {}
impl ToTokens for VerbatimFn { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (& self . attrs) ; self . vis . to_tokens (tokens) ; self . defaultness . to_tokens (tokens) ; self . sig . to_tokens (tokens) ; self . semi_token . to_tokens (tokens) ; } }
};
}
