// Generated macro for impl_1624 (impl)
macro_rules! Depcrate_syncimpl_1624 {
() => {
// Module: crate::sync
// Provides: {"impl_1624"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_array" , since = "1.74.0")] impl < T , const N : usize > From < [T ; N] > for Arc < [T] > { # [doc = " Converts a [`[T; N]`](prim@array) into an `Arc<[T]>`."] # [doc = ""] # [doc = " The conversion moves the array into a newly allocated `Arc`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let original: [i32; 3] = [1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : [T ; N]) -> Arc < [T] > { Arc :: < [T ; N] > :: from (v) } }
};
}
