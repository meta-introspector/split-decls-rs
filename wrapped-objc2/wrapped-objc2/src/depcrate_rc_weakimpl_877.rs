// Generated macro for impl_877 (impl)
macro_rules! Depcrate_rc_weakimpl_877 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_877"}
// Dependencies: {}
impl < T : Message > From < & T > for Weak < T > { # [inline] fn from (obj : & T) -> Self { Weak :: new (obj) } }
};
}
