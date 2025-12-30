// Generated macro for impl_829 (impl)
macro_rules! Depcrate_spanimpl_829 {
() => {
// Module: crate::span
// Provides: {"impl_829"}
// Dependencies: {}
impl < 'a > From < (Unit , SpanRelativeTo < 'a >) > for SpanTotal < 'a > { # [inline] fn from ((unit , relative) : (Unit , SpanRelativeTo < 'a >)) -> SpanTotal < 'a > { SpanTotal :: from (unit) . relative (relative) } }
};
}
