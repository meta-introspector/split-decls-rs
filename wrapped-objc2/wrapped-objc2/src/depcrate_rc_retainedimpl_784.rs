// Generated macro for impl_784 (impl)
macro_rules! Depcrate_rc_retainedimpl_784 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_784"}
// Dependencies: {}
impl < T : ClassType + 'static > From < Retained < T > > for Retained < AnyObject > { # [doc = " Convert the object to `AnyObject`."] # [inline] fn from (obj : Retained < T >) -> Self { unsafe { Retained :: cast_unchecked (obj) } } }
};
}
