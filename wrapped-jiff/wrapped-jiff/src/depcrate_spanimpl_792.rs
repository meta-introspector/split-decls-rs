// Generated macro for impl_792 (impl)
macro_rules! Depcrate_spanimpl_792 {
() => {
// Module: crate::span
// Provides: {"impl_792"}
// Dependencies: {}
impl < 'a > From < & 'a Span > for SpanArithmetic < 'static > { fn from (span : & 'a Span) -> SpanArithmetic < 'static > { let duration = Duration :: from (* span) ; SpanArithmetic { duration , relative : None } } }
};
}
