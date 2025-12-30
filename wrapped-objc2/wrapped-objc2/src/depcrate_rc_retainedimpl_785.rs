// Generated macro for impl_785 (impl)
macro_rules! Depcrate_rc_retainedimpl_785 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_785"}
// Dependencies: {}
impl < P : ? Sized + 'static > From < Retained < ProtocolObject < P > > > for Retained < AnyObject > { # [doc = " Convert the protocol object to `AnyObject`."] # [inline] fn from (obj : Retained < ProtocolObject < P > >) -> Self { unsafe { Retained :: cast_unchecked (obj) } } }
};
}
