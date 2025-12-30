// Generated macro for impl_90 (impl)
macro_rules! Depcrate_errorimpl_90 {
() => {
// Module: crate::error
// Provides: {"impl_90"}
// Dependencies: {}
impl ToTokens for SpanError { fn to_tokens (& self , tokens : & mut TokenStream2) { let span = self . span . unwrap_or_else (Span :: call_site) ; let token_stream_err = syn :: Error :: new (span , self . err . clone ()) . to_compile_error () ; token_stream_err . to_tokens (tokens) ; } }
};
}
