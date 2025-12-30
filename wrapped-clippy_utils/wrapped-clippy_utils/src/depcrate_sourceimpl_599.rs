// Generated macro for impl_599 (impl)
macro_rules! Depcrate_sourceimpl_599 {
() => {
// Module: crate::source
// Provides: {"impl_599"}
// Dependencies: {}
impl IntoSpan for SpanData { fn into_span (self) -> Span { self . span () } fn with_ctxt (self , ctxt : SyntaxContext) -> Span { Span :: new (self . lo , self . hi , ctxt , self . parent) } }
};
}
