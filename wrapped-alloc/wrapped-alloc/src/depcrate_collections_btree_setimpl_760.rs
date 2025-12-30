// Generated macro for impl_760 (impl)
macro_rules! Depcrate_collections_btree_setimpl_760 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_760"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T , A > Default for IntoIter < T , A > where A : Allocator + Default + Clone , { # [doc = " Creates an empty `btree_set::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_set;"] # [doc = " let iter: btree_set::IntoIter<u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IntoIter { iter : Default :: default () } } }
};
}
