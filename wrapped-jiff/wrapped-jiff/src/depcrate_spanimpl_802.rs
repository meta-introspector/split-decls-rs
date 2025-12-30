// Generated macro for impl_802 (impl)
macro_rules! Depcrate_spanimpl_802 {
() => {
// Module: crate::span
// Provides: {"impl_802"}
// Dependencies: {}
impl From < (SignedDuration , Date) > for SpanArithmetic < 'static > { # [inline] fn from ((duration , date) : (SignedDuration , Date) ,) -> SpanArithmetic < 'static > { SpanArithmetic :: from (duration) . relative (date) } }
};
}
