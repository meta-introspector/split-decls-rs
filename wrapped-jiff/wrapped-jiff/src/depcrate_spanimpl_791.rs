// Generated macro for impl_791 (impl)
macro_rules! Depcrate_spanimpl_791 {
() => {
// Module: crate::span
// Provides: {"impl_791"}
// Dependencies: {}
impl From < Span > for SpanArithmetic < 'static > { fn from (span : Span) -> SpanArithmetic < 'static > { let duration = Duration :: from (span) ; SpanArithmetic { duration , relative : None } } }
};
}
