// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_rcimpl_1289 {
() => {
// Module: crate::rc
// Provides: {"impl_1289"}
// Dependencies: {}
impl < T : ? Sized > RcInnerPtr for RcInner < T > { # [inline (always)] fn weak_ref (& self) -> & Cell < usize > { & self . weak } # [inline (always)] fn strong_ref (& self) -> & Cell < usize > { & self . strong } }
};
}
