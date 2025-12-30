// Generated macro for impl_1971 (impl)
macro_rules! Depcrate_vecimpl_1971 {
() => {
// Module: crate::vec
// Provides: {"impl_1971"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "vec_from_mut" , since = "1.19.0")] impl < T : Clone > From < & mut [T] > for Vec < T > { # [doc = " Allocates a `Vec<T>` and fills it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Vec::from(&mut [1, 2, 3][..]), vec![1, 2, 3]);"] # [doc = " ```"] # [track_caller] fn from (s : & mut [T]) -> Vec < T > { s . to_vec () } }
};
}
