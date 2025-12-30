// Generated macro for impl_1667 (impl)
macro_rules! Depcrate_syncimpl_1667 {
() => {
// Module: crate::sync
// Provides: {"impl_1667"}
// Dependencies: {}
impl < T , A : Allocator > UniqueArc < T , A > { # [doc = " Creates a new `UniqueArc` in the provided allocator."] # [doc = ""] # [doc = " Weak references to this `UniqueArc` can be created with [`UniqueArc::downgrade`]. Upgrading"] # [doc = " these weak references will fail before the `UniqueArc` has been converted into an [`Arc`]."] # [doc = " After converting the `UniqueArc` into an [`Arc`], any weak references created beforehand will"] # [doc = " point to the new [`Arc`]."] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "unique_rc_arc" , issue = "112566")] # [must_use] pub fn new_in (data : T , alloc : A) -> Self { let (ptr , alloc) = Box :: into_unique (Box :: new_in (ArcInner { strong : atomic :: AtomicUsize :: new (0) , weak : atomic :: AtomicUsize :: new (1) , data , } , alloc ,)) ; Self { ptr : ptr . into () , _marker : PhantomData , _marker2 : PhantomData , alloc } } }
};
}
