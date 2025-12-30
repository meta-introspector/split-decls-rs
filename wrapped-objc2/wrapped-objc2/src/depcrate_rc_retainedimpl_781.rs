// Generated macro for impl_781 (impl)
macro_rules! Depcrate_rc_retainedimpl_781 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_781"}
// Dependencies: {}
impl < T : ? Sized > Deref for Retained < T > { type Target = T ; # [doc = " Obtain an immutable reference to the object."] # [inline] fn deref (& self) -> & T { unsafe { self . ptr . as_ref () } } }
};
}
