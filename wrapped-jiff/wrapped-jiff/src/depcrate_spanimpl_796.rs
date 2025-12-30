// Generated macro for impl_796 (impl)
macro_rules! Depcrate_spanimpl_796 {
() => {
// Module: crate::span
// Provides: {"impl_796"}
// Dependencies: {}
impl < 'a > From < (Span , SpanRelativeTo < 'a >) > for SpanArithmetic < 'a > { # [inline] fn from ((span , relative) : (Span , SpanRelativeTo < 'a >) ,) -> SpanArithmetic < 'a > { SpanArithmetic :: from (span) . relative (relative) } }
};
}
