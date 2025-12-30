// Generated macro for impl_821 (impl)
macro_rules! Depcrate_spanimpl_821 {
() => {
// Module: crate::span
// Provides: {"impl_821"}
// Dependencies: {}
impl < 'a , 'b > From < (& 'a Span , SpanRelativeTo < 'b >) > for SpanCompare < 'b > { # [inline] fn from ((span , relative) : (& 'a Span , SpanRelativeTo < 'b >) ,) -> SpanCompare < 'b > { SpanCompare :: from (span) . relative (relative) } }
};
}
