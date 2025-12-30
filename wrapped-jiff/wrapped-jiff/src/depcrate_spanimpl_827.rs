// Generated macro for impl_827 (impl)
macro_rules! Depcrate_spanimpl_827 {
() => {
// Module: crate::span
// Provides: {"impl_827"}
// Dependencies: {}
impl From < (Unit , DateTime) > for SpanTotal < 'static > { # [inline] fn from ((unit , datetime) : (Unit , DateTime)) -> SpanTotal < 'static > { SpanTotal :: from (unit) . relative (datetime) } }
};
}
