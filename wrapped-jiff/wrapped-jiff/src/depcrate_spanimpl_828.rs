// Generated macro for impl_828 (impl)
macro_rules! Depcrate_spanimpl_828 {
() => {
// Module: crate::span
// Provides: {"impl_828"}
// Dependencies: {}
impl < 'a > From < (Unit , & 'a Zoned) > for SpanTotal < 'a > { # [inline] fn from ((unit , zoned) : (Unit , & 'a Zoned)) -> SpanTotal < 'a > { SpanTotal :: from (unit) . relative (zoned) } }
};
}
