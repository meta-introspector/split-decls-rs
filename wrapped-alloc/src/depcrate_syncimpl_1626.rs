// Generated macro for impl_1626 (impl)
macro_rules! Depcrate_syncimpl_1626 {
() => {
// Module: crate::sync
// Provides: {"impl_1626"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl < T : Clone > From < & mut [T] > for Arc < [T] > { # [doc = " Allocates a reference-counted slice and fills it by cloning `v`'s items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let mut original = [1, 2, 3];"] # [doc = " let original: &mut [i32] = &mut original;"] # [doc = " let shared: Arc<[i32]> = Arc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & mut [T]) -> Arc < [T] > { Arc :: from (& * v) } }
};
}
