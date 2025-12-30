// Generated macro for impl_878 (impl)
macro_rules! Depcrate_rc_weakimpl_878 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_878"}
// Dependencies: {}
impl < T : Message > From < & Retained < T > > for Weak < T > { # [inline] fn from (obj : & Retained < T >) -> Self { Weak :: from_retained (obj) } }
};
}
