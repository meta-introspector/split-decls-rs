// Generated macro for impl_1278 (impl)
macro_rules! Depcrate_rcimpl_1278 {
() => {
// Module: crate::rc
// Provides: {"impl_1278"}
// Dependencies: {}
impl < T , A : Allocator > Weak < T , A > { # [doc = " Constructs a new `Weak<T>`, without allocating any memory, technically in the provided"] # [doc = " allocator."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Weak::new();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] # [inline] # [unstable (feature = "allocator_api" , issue = "32838")] pub fn new_in (alloc : A) -> Weak < T , A > { Weak { ptr : NonNull :: without_provenance (NonZeroUsize :: MAX) , alloc } } }
};
}
