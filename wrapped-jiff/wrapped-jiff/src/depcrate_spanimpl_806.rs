// Generated macro for impl_806 (impl)
macro_rules! Depcrate_spanimpl_806 {
() => {
// Module: crate::span
// Provides: {"impl_806"}
// Dependencies: {}
impl From < (UnsignedDuration , Date) > for SpanArithmetic < 'static > { # [inline] fn from ((duration , date) : (UnsignedDuration , Date) ,) -> SpanArithmetic < 'static > { SpanArithmetic :: from (duration) . relative (date) } }
};
}
