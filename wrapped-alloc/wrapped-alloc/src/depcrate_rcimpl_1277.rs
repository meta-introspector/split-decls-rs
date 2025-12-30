// Generated macro for impl_1277 (impl)
macro_rules! Depcrate_rcimpl_1277 {
() => {
// Module: crate::rc
// Provides: {"impl_1277"}
// Dependencies: {}
impl < T > Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating any memory."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Weak::new();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] # [inline] # [stable (feature = "downgraded_weak" , since = "1.10.0")] # [rustc_const_stable (feature = "const_weak_new" , since = "1.73.0")] # [must_use] pub const fn new () -> Weak < T > { Weak { ptr : NonNull :: without_provenance (NonZeroUsize :: MAX) , alloc : Global } } }
};
}
