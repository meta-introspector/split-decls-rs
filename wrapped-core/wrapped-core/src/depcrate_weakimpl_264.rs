// Generated macro for impl_264 (impl)
macro_rules! Depcrate_weakimpl_264 {
() => {
// Module: crate::weak
// Provides: {"impl_264"}
// Dependencies: {}
impl < I : Interface > Weak < I > { # [doc = " Creates a new `Weak` object without any backing object."] pub const fn new () -> Self { Self (None , PhantomData) } # [doc = " Attempts to upgrade the weak reference to a strong reference."] pub fn upgrade (& self) -> Option < I > { self . 0 . as_ref () . and_then (| inner | unsafe { inner . Resolve () . ok () }) } pub (crate) fn downgrade (source : & imp :: IWeakReferenceSource) -> Self { let reference = unsafe { source . GetWeakReference () . ok () } ; Self (reference , PhantomData) } }
};
}
