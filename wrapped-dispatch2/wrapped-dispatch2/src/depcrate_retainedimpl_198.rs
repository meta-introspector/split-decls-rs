// Generated macro for impl_198 (impl)
macro_rules! Depcrate_retainedimpl_198 {
() => {
// Module: crate::retained
// Provides: {"impl_198"}
// Dependencies: {}
impl < T : ? Sized > Deref for DispatchRetained < T > { type Target = T ; # [doc = " Obtain a reference to the object."] # [inline] fn deref (& self) -> & T { unsafe { self . ptr . as_ref () } } }
};
}
