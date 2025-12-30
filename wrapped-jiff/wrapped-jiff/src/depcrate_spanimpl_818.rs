// Generated macro for impl_818 (impl)
macro_rules! Depcrate_spanimpl_818 {
() => {
// Module: crate::span
// Provides: {"impl_818"}
// Dependencies: {}
impl < 'a > From < (& 'a Span , Date) > for SpanCompare < 'static > { # [inline] fn from ((span , date) : (& 'a Span , Date)) -> SpanCompare < 'static > { SpanCompare :: from (span) . relative (date) } }
};
}
