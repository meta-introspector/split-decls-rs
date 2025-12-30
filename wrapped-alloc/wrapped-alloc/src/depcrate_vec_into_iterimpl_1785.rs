// Generated macro for impl_1785 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1785 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1785"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T , A > Default for IntoIter < T , A > where A : Allocator + Default , { # [doc = " Creates an empty `vec::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::vec;"] # [doc = " let iter: vec::IntoIter<u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " assert_eq!(iter.as_slice(), &[]);"] # [doc = " ```"] fn default () -> Self { super :: Vec :: new_in (Default :: default ()) . into_iter () } }
};
}
