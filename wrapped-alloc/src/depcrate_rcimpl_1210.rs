// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_rcimpl_1210 {
() => {
// Module: crate::rc
// Provides: {"impl_1210"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Rc < T , A > { # [inline (always)] fn inner (& self) -> & RcInner < T > { unsafe { self . ptr . as_ref () } } # [inline] fn into_inner_with_allocator (this : Self) -> (NonNull < RcInner < T > > , A) { let this = mem :: ManuallyDrop :: new (this) ; (this . ptr , unsafe { ptr :: read (& this . alloc) }) } # [inline] unsafe fn from_inner_in (ptr : NonNull < RcInner < T > > , alloc : A) -> Self { Self { ptr , phantom : PhantomData , alloc } } # [inline] unsafe fn from_ptr_in (ptr : * mut RcInner < T > , alloc : A) -> Self { unsafe { Self :: from_inner_in (NonNull :: new_unchecked (ptr) , alloc) } } # [inline (never)] unsafe fn drop_slow (& mut self) { let _weak = Weak { ptr : self . ptr , alloc : & self . alloc } ; unsafe { ptr :: drop_in_place (& mut (* self . ptr . as_ptr ()) . value) ; } } }
};
}
