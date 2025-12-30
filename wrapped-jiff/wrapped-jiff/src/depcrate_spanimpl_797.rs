// Generated macro for impl_797 (impl)
macro_rules! Depcrate_spanimpl_797 {
() => {
// Module: crate::span
// Provides: {"impl_797"}
// Dependencies: {}
impl < 'a > From < (& 'a Span , Date) > for SpanArithmetic < 'static > { # [inline] fn from ((span , date) : (& 'a Span , Date)) -> SpanArithmetic < 'static > { SpanArithmetic :: from (span) . relative (date) } }
};
}
