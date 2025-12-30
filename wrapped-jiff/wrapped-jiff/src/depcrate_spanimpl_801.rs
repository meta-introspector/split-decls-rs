// Generated macro for impl_801 (impl)
macro_rules! Depcrate_spanimpl_801 {
() => {
// Module: crate::span
// Provides: {"impl_801"}
// Dependencies: {}
impl From < SignedDuration > for SpanArithmetic < 'static > { fn from (duration : SignedDuration) -> SpanArithmetic < 'static > { let duration = Duration :: from (duration) ; SpanArithmetic { duration , relative : None } } }
};
}
