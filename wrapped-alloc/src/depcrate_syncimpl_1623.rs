// Generated macro for impl_1623 (impl)
macro_rules! Depcrate_syncimpl_1623 {
() => {
// Module: crate::sync
// Provides: {"impl_1623"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "from_for_ptrs" , since = "1.6.0")] impl < T > From < T > for Arc < T > { # [doc = " Converts a `T` into an `Arc<T>`"] # [doc = ""] # [doc = " The conversion moves the value into a"] # [doc = " newly allocated `Arc`. It is equivalent to"] # [doc = " calling `Arc::new(t)`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use std::sync::Arc;"] # [doc = " let x = 5;"] # [doc = " let arc = Arc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Arc::from(x), arc);"] # [doc = " ```"] fn from (t : T) -> Self { Arc :: new (t) } }
};
}
