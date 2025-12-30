// Generated macro for impl_270 (impl)
macro_rules! Depcrate_vecimpl_270 {
() => {
// Module: crate::vec
// Provides: {"impl_270"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone > From < & mut [T] > for Vec < T > { # [doc = " Allocate a `Vec<T>` and fill it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " assert_eq!(Vec::from(&mut [1, 2, 3][..]), vec![1, 2, 3]);"] # [doc = " ```"] # [inline (always)] fn from (s : & mut [T]) -> Vec < T > { let mut vec = Vec :: with_capacity (s . len ()) ; vec . extend_from_slice (s) ; vec } }
};
}
