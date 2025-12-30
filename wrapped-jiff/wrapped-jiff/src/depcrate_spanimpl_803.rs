// Generated macro for impl_803 (impl)
macro_rules! Depcrate_spanimpl_803 {
() => {
// Module: crate::span
// Provides: {"impl_803"}
// Dependencies: {}
impl From < (SignedDuration , DateTime) > for SpanArithmetic < 'static > { # [inline] fn from ((duration , datetime) : (SignedDuration , DateTime) ,) -> SpanArithmetic < 'static > { SpanArithmetic :: from (duration) . relative (datetime) } }
};
}
