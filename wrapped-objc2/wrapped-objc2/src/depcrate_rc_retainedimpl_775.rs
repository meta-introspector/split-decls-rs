// Generated macro for impl_775 (impl)
macro_rules! Depcrate_rc_retainedimpl_775 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_775"}
// Dependencies: {}
impl < T : ? Sized > Retained < T > { # [inline] pub (crate) unsafe fn new_nonnull (ptr : NonNull < T >) -> Self { Self { ptr , item : PhantomData , notunwindsafe : PhantomData , } } }
};
}
