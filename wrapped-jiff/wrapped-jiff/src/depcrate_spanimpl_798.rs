// Generated macro for impl_798 (impl)
macro_rules! Depcrate_spanimpl_798 {
() => {
// Module: crate::span
// Provides: {"impl_798"}
// Dependencies: {}
impl < 'a > From < (& 'a Span , DateTime) > for SpanArithmetic < 'static > { # [inline] fn from ((span , datetime) : (& 'a Span , DateTime) ,) -> SpanArithmetic < 'static > { SpanArithmetic :: from (span) . relative (datetime) } }
};
}
