// Generated macro for impl_567 (impl)
macro_rules! Depcrate_sourceimpl_567 {
() => {
// Module: crate::source
// Provides: {"impl_567"}
// Dependencies: {}
impl IntoSpan for SpanData { fn into_span (self) -> Span { self . span () } fn with_ctxt (self , ctxt : SyntaxContext) -> Span { Span :: new (self . lo , self . hi , ctxt , self . parent) } }
};
}
