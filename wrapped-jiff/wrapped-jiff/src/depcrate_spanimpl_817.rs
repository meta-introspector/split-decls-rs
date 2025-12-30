// Generated macro for impl_817 (impl)
macro_rules! Depcrate_spanimpl_817 {
() => {
// Module: crate::span
// Provides: {"impl_817"}
// Dependencies: {}
impl < 'a > From < (Span , SpanRelativeTo < 'a >) > for SpanCompare < 'a > { # [inline] fn from ((span , relative) : (Span , SpanRelativeTo < 'a >)) -> SpanCompare < 'a > { SpanCompare :: from (span) . relative (relative) } }
};
}
