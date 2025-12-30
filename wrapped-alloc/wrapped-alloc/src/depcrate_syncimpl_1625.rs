// Generated macro for impl_1625 (impl)
macro_rules! Depcrate_syncimpl_1625 {
() => {
// Module: crate::sync
// Provides: {"impl_1625"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl < T : Clone > From < & [T] > for Arc < [T] > { # [doc = " Allocates a reference-counted slice and fills it by cloning `v`'s items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let original: &[i32] = &[1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & [T]) -> Arc < [T] > { < Self as ArcFromSlice < T > > :: from_slice (v) } }
};
}
