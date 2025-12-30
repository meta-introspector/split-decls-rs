// Generated macro for impl_1556 (impl)
macro_rules! Depcrate_syncimpl_1556 {
() => {
// Module: crate::sync
// Provides: {"impl_1556"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Arc < T , A > { # [inline] fn into_inner_with_allocator (this : Self) -> (NonNull < ArcInner < T > > , A) { let this = mem :: ManuallyDrop :: new (this) ; (this . ptr , unsafe { ptr :: read (& this . alloc) }) } # [inline] unsafe fn from_inner_in (ptr : NonNull < ArcInner < T > > , alloc : A) -> Self { Self { ptr , phantom : PhantomData , alloc } } # [inline] unsafe fn from_ptr_in (ptr : * mut ArcInner < T > , alloc : A) -> Self { unsafe { Self :: from_inner_in (NonNull :: new_unchecked (ptr) , alloc) } } }
};
}
