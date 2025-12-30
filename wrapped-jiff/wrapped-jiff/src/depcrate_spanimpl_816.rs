// Generated macro for impl_816 (impl)
macro_rules! Depcrate_spanimpl_816 {
() => {
// Module: crate::span
// Provides: {"impl_816"}
// Dependencies: {}
impl < 'a > From < (Span , & 'a Zoned) > for SpanCompare < 'a > { # [inline] fn from ((span , zoned) : (Span , & 'a Zoned)) -> SpanCompare < 'a > { SpanCompare :: from (span) . relative (zoned) } }
};
}
