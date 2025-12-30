// Generated macro for impl_799 (impl)
macro_rules! Depcrate_spanimpl_799 {
() => {
// Module: crate::span
// Provides: {"impl_799"}
// Dependencies: {}
impl < 'a , 'b > From < (& 'a Span , & 'b Zoned) > for SpanArithmetic < 'b > { # [inline] fn from ((span , zoned) : (& 'a Span , & 'b Zoned)) -> SpanArithmetic < 'b > { SpanArithmetic :: from (span) . relative (zoned) } }
};
}
