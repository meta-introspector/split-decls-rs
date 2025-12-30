// Generated macro for impl_815 (impl)
macro_rules! Depcrate_spanimpl_815 {
() => {
// Module: crate::span
// Provides: {"impl_815"}
// Dependencies: {}
impl From < (Span , DateTime) > for SpanCompare < 'static > { # [inline] fn from ((span , datetime) : (Span , DateTime)) -> SpanCompare < 'static > { SpanCompare :: from (span) . relative (datetime) } }
};
}
