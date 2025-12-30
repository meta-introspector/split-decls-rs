// Generated macro for impl_129 (impl)
macro_rules! Depcrate_common_span_containerimpl_129 {
() => {
// Module: crate::common::span_container
// Provides: {"impl_129"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for SpanContainer < T > { fn to_tokens (& self , tokens : & mut TokenStream) { self . val . to_tokens (tokens) } }
};
}
