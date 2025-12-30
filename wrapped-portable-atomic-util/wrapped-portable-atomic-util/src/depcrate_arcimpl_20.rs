// Generated macro for impl_20 (impl)
macro_rules! Depcrate_arcimpl_20 {
() => {
// Module: crate::arc
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : ? Sized > Arc < T > { # [inline] fn into_inner_non_null (this : Self) -> NonNull < ArcInner < T > > { let this = mem :: ManuallyDrop :: new (this) ; this . ptr } # [inline] unsafe fn from_inner (ptr : NonNull < ArcInner < T > >) -> Self { Self { ptr , phantom : PhantomData } } # [inline] unsafe fn from_ptr (ptr : * mut ArcInner < T >) -> Self { unsafe { Self :: from_inner (NonNull :: new_unchecked (ptr)) } } }
};
}
