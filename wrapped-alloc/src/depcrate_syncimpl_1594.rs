// Generated macro for impl_1594 (impl)
macro_rules! Depcrate_syncimpl_1594 {
() => {
// Module: crate::sync
// Provides: {"impl_1594"}
// Dependencies: {}
impl < T > Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating any memory."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Weak::new();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] # [inline] # [stable (feature = "downgraded_weak" , since = "1.10.0")] # [rustc_const_stable (feature = "const_weak_new" , since = "1.73.0")] # [must_use] pub const fn new () -> Weak < T > { Weak { ptr : NonNull :: without_provenance (NonZeroUsize :: MAX) , alloc : Global } } }
};
}
