// Generated macro for impl_1973 (impl)
macro_rules! Depcrate_vecimpl_1973 {
() => {
// Module: crate::vec
// Provides: {"impl_1973"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "vec_from_array_ref" , since = "1.74.0")] impl < T : Clone , const N : usize > From < & mut [T ; N] > for Vec < T > { # [doc = " Allocates a `Vec<T>` and fills it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Vec::from(&mut [1, 2, 3]), vec![1, 2, 3]);"] # [doc = " ```"] # [track_caller] fn from (s : & mut [T ; N]) -> Vec < T > { Self :: from (s . as_mut_slice ()) } }
};
}
