// Generated macro for impl_1595 (impl)
macro_rules! Depcrate_syncimpl_1595 {
() => {
// Module: crate::sync
// Provides: {"impl_1595"}
// Dependencies: {}
impl < T , A : Allocator > Weak < T , A > { # [doc = " Constructs a new `Weak<T, A>`, without allocating any memory, technically in the provided"] # [doc = " allocator."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(allocator_api)]"] # [doc = ""] # [doc = " use std::sync::Weak;"] # [doc = " use std::alloc::System;"] # [doc = ""] # [doc = " let empty: Weak<i64, _> = Weak::new_in(System);"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] # [inline] # [unstable (feature = "allocator_api" , issue = "32838")] pub fn new_in (alloc : A) -> Weak < T , A > { Weak { ptr : NonNull :: without_provenance (NonZeroUsize :: MAX) , alloc } } }
};
}
