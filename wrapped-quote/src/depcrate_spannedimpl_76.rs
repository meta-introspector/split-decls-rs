// Generated macro for impl_76 (impl)
macro_rules! Depcrate_spannedimpl_76 {
() => {
// Module: crate::spanned
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : ? Sized + ToTokens > Spanned for T { fn __span (& self) -> Span { join_spans (self . into_token_stream ()) } }
};
}
