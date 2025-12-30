// Generated macro for impl_879 (impl)
macro_rules! Depcrate_rc_weakimpl_879 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_879"}
// Dependencies: {}
impl < T : Message > From < Retained < T > > for Weak < T > { # [inline] fn from (obj : Retained < T >) -> Self { Weak :: from_retained (& obj) } }
};
}
