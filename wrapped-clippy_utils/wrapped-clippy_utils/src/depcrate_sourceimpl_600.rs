// Generated macro for impl_600 (impl)
macro_rules! Depcrate_sourceimpl_600 {
() => {
// Module: crate::source
// Provides: {"impl_600"}
// Dependencies: {}
impl IntoSpan for Range < BytePos > { fn into_span (self) -> Span { Span :: with_root_ctxt (self . start , self . end) } fn with_ctxt (self , ctxt : SyntaxContext) -> Span { Span :: new (self . start , self . end , ctxt , None) } }
};
}
