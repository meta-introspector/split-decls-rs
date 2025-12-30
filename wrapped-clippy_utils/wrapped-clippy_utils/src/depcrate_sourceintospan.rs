// Generated macro for IntoSpan (trait)
macro_rules! Depcrate_sourceIntoSpan {
() => {
// Module: crate::source
// Provides: {"IntoSpan"}
// Dependencies: {}
# [doc = " Conversion of a value into a `Span`"] pub trait IntoSpan : Sized { fn into_span (self) -> Span ; fn with_ctxt (self , ctxt : SyntaxContext) -> Span ; }
};
}
