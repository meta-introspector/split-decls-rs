// Generated macro for impl_807 (impl)
macro_rules! Depcrate_spanimpl_807 {
() => {
// Module: crate::span
// Provides: {"impl_807"}
// Dependencies: {}
impl From < (UnsignedDuration , DateTime) > for SpanArithmetic < 'static > { # [inline] fn from ((duration , datetime) : (UnsignedDuration , DateTime) ,) -> SpanArithmetic < 'static > { SpanArithmetic :: from (duration) . relative (datetime) } }
};
}
