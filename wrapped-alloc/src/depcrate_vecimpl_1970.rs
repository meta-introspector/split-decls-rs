// Generated macro for impl_1970 (impl)
macro_rules! Depcrate_vecimpl_1970 {
() => {
// Module: crate::vec
// Provides: {"impl_1970"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone > From < & [T] > for Vec < T > { # [doc = " Allocates a `Vec<T>` and fills it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Vec::from(&[1, 2, 3][..]), vec![1, 2, 3]);"] # [doc = " ```"] # [track_caller] fn from (s : & [T]) -> Vec < T > { s . to_vec () } }
};
}
