// Generated macro for impl_1290 (impl)
macro_rules! Depcrate_rcimpl_1290 {
() => {
// Module: crate::rc
// Provides: {"impl_1290"}
// Dependencies: {}
impl < 'a > RcInnerPtr for WeakInner < 'a > { # [inline (always)] fn weak_ref (& self) -> & Cell < usize > { self . weak } # [inline (always)] fn strong_ref (& self) -> & Cell < usize > { self . strong } }
};
}
