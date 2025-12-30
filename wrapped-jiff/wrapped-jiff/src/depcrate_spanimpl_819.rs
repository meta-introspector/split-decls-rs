// Generated macro for impl_819 (impl)
macro_rules! Depcrate_spanimpl_819 {
() => {
// Module: crate::span
// Provides: {"impl_819"}
// Dependencies: {}
impl < 'a > From < (& 'a Span , DateTime) > for SpanCompare < 'static > { # [inline] fn from ((span , datetime) : (& 'a Span , DateTime)) -> SpanCompare < 'static > { SpanCompare :: from (span) . relative (datetime) } }
};
}
