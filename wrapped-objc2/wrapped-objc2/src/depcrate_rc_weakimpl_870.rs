// Generated macro for impl_870 (impl)
macro_rules! Depcrate_rc_weakimpl_870 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_870"}
// Dependencies: {}
impl < T : Message > Default for Weak < T > { # [doc = " Constructs a new weak pointer that doesn't reference any object."] # [doc = ""] # [doc = " Calling [`Self::load`] on the return value always gives [`None`]."] # [inline] fn default () -> Self { unsafe { Self :: new_inner (ptr :: null ()) } } }
};
}
