// Generated macro for impl_136 (impl)
macro_rules! Depcrate_utils_errorimpl_136 {
() => {
// Module: crate::utils::error
// Provides: {"impl_136"}
// Dependencies: {}
impl < T : quote :: ToTokens > ToTokensError for T { fn EXPECTED_Span_OR_ToTokens < D : std :: fmt :: Display > (& self , msg : D) -> syn :: Error { syn :: Error :: new_spanned (self . to_token_stream () , msg) } }
};
}
