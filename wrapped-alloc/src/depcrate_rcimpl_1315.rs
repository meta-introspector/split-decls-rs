// Generated macro for impl_1315 (impl)
macro_rules! Depcrate_rcimpl_1315 {
() => {
// Module: crate::rc
// Provides: {"impl_1315"}
// Dependencies: {}
impl < T , A : Allocator > UniqueRc < T , A > { # [doc = " Creates a new `UniqueRc` in the provided allocator."] # [doc = ""] # [doc = " Weak references to this `UniqueRc` can be created with [`UniqueRc::downgrade`]. Upgrading"] # [doc = " these weak references will fail before the `UniqueRc` has been converted into an [`Rc`]."] # [doc = " After converting the `UniqueRc` into an [`Rc`], any weak references created beforehand will"] # [doc = " point to the new [`Rc`]."] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "unique_rc_arc" , issue = "112566")] pub fn new_in (value : T , alloc : A) -> Self { let (ptr , alloc) = Box :: into_unique (Box :: new_in (RcInner { strong : Cell :: new (0) , weak : Cell :: new (1) , value , } , alloc ,)) ; Self { ptr : ptr . into () , _marker : PhantomData , _marker2 : PhantomData , alloc } } }
};
}
