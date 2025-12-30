// Generated macro for impl_794 (impl)
macro_rules! Depcrate_spanimpl_794 {
() => {
// Module: crate::span
// Provides: {"impl_794"}
// Dependencies: {}
impl From < (Span , DateTime) > for SpanArithmetic < 'static > { # [inline] fn from ((span , datetime) : (Span , DateTime)) -> SpanArithmetic < 'static > { SpanArithmetic :: from (span) . relative (datetime) } }
};
}
