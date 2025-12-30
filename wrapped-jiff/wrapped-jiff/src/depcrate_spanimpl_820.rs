// Generated macro for impl_820 (impl)
macro_rules! Depcrate_spanimpl_820 {
() => {
// Module: crate::span
// Provides: {"impl_820"}
// Dependencies: {}
impl < 'a , 'b > From < (& 'a Span , & 'b Zoned) > for SpanCompare < 'b > { # [inline] fn from ((span , zoned) : (& 'a Span , & 'b Zoned)) -> SpanCompare < 'b > { SpanCompare :: from (span) . relative (zoned) } }
};
}
