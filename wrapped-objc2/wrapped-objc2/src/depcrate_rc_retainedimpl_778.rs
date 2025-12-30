// Generated macro for impl_778 (impl)
macro_rules! Depcrate_rc_retainedimpl_778 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_778"}
// Dependencies: {}
impl < T : ClassType + 'static > Retained < T > where T :: Super : 'static , { # [doc = " Convert the object into its superclass."] # [inline] pub fn into_super (self) -> Retained < T :: Super > { unsafe { Self :: cast_unchecked :: < T :: Super > (self) } } }
};
}
