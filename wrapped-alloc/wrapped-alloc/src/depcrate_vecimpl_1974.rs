// Generated macro for impl_1974 (impl)
macro_rules! Depcrate_vecimpl_1974 {
() => {
// Module: crate::vec
// Provides: {"impl_1974"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "vec_from_array" , since = "1.44.0")] impl < T , const N : usize > From < [T ; N] > for Vec < T > { # [doc = " Allocates a `Vec<T>` and moves `s`'s items into it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Vec::from([1, 2, 3]), vec![1, 2, 3]);"] # [doc = " ```"] # [track_caller] fn from (s : [T ; N]) -> Vec < T > { < [T] > :: into_vec (Box :: new (s)) } }
};
}
