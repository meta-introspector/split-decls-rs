// Generated macro for impl_793 (impl)
macro_rules! Depcrate_spanimpl_793 {
() => {
// Module: crate::span
// Provides: {"impl_793"}
// Dependencies: {}
impl From < (Span , Date) > for SpanArithmetic < 'static > { # [inline] fn from ((span , date) : (Span , Date)) -> SpanArithmetic < 'static > { SpanArithmetic :: from (span) . relative (date) } }
};
}
