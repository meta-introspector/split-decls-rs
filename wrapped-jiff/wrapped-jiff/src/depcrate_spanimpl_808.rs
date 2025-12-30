// Generated macro for impl_808 (impl)
macro_rules! Depcrate_spanimpl_808 {
() => {
// Module: crate::span
// Provides: {"impl_808"}
// Dependencies: {}
impl < 'a > From < (UnsignedDuration , & 'a Zoned) > for SpanArithmetic < 'a > { # [inline] fn from ((duration , zoned) : (UnsignedDuration , & 'a Zoned) ,) -> SpanArithmetic < 'a > { SpanArithmetic :: from (duration) . relative (zoned) } }
};
}
