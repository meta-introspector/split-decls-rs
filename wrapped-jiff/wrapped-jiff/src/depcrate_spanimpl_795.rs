// Generated macro for impl_795 (impl)
macro_rules! Depcrate_spanimpl_795 {
() => {
// Module: crate::span
// Provides: {"impl_795"}
// Dependencies: {}
impl < 'a > From < (Span , & 'a Zoned) > for SpanArithmetic < 'a > { # [inline] fn from ((span , zoned) : (Span , & 'a Zoned)) -> SpanArithmetic < 'a > { SpanArithmetic :: from (span) . relative (zoned) } }
};
}
