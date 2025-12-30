// Generated macro for impl_800 (impl)
macro_rules! Depcrate_spanimpl_800 {
() => {
// Module: crate::span
// Provides: {"impl_800"}
// Dependencies: {}
impl < 'a , 'b > From < (& 'a Span , SpanRelativeTo < 'b >) > for SpanArithmetic < 'b > { # [inline] fn from ((span , relative) : (& 'a Span , SpanRelativeTo < 'b >) ,) -> SpanArithmetic < 'b > { SpanArithmetic :: from (span) . relative (relative) } }
};
}
