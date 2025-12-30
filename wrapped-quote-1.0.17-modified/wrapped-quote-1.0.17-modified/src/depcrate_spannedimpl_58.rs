// Generated macro for impl_58 (impl)
macro_rules! Depcrate_spannedimpl_58 {
() => {
// Module: crate::spanned
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > Spanned for T { fn __span (& self) -> Span { join_spans (self . into_token_stream ()) } }
};
}
