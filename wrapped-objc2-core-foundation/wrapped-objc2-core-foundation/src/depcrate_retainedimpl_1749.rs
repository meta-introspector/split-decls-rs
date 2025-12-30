// Generated macro for impl_1749 (impl)
macro_rules! Depcrate_retainedimpl_1749 {
() => {
// Module: crate::retained
// Provides: {"impl_1749"}
// Dependencies: {}
impl < T : ? Sized > Deref for CFRetained < T > { type Target = T ; # [doc = " Obtain a reference to the type."] # [inline] fn deref (& self) -> & T { unsafe { self . ptr . as_ref () } } }
};
}
