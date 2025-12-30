// Generated macro for impl_804 (impl)
macro_rules! Depcrate_spanimpl_804 {
() => {
// Module: crate::span
// Provides: {"impl_804"}
// Dependencies: {}
impl < 'a > From < (SignedDuration , & 'a Zoned) > for SpanArithmetic < 'a > { # [inline] fn from ((duration , zoned) : (SignedDuration , & 'a Zoned) ,) -> SpanArithmetic < 'a > { SpanArithmetic :: from (duration) . relative (zoned) } }
};
}
