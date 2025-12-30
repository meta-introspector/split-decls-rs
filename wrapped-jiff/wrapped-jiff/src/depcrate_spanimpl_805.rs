// Generated macro for impl_805 (impl)
macro_rules! Depcrate_spanimpl_805 {
() => {
// Module: crate::span
// Provides: {"impl_805"}
// Dependencies: {}
impl From < UnsignedDuration > for SpanArithmetic < 'static > { fn from (duration : UnsignedDuration) -> SpanArithmetic < 'static > { let duration = Duration :: from (duration) ; SpanArithmetic { duration , relative : None } } }
};
}
