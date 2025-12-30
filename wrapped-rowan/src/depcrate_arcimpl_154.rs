// Generated macro for impl_154 (impl)
macro_rules! Depcrate_arcimpl_154 {
() => {
// Module: crate::arc
// Provides: {"impl_154"}
// Dependencies: {}
impl < T : ? Sized > Clone for Arc < T > { # [inline] fn clone (& self) -> Self { let old_size = self . inner () . count . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { std :: process :: abort () ; } unsafe { Arc { p : ptr :: NonNull :: new_unchecked (self . ptr ()) , phantom : PhantomData } } } }
};
}
