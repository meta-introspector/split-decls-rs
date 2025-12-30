// Generated macro for impl_756 (impl)
macro_rules! Depcrate_collections_btree_setimpl_756 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_756"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T > Default for Iter < '_ , T > { # [doc = " Creates an empty `btree_set::Iter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_set;"] # [doc = " let iter: btree_set::Iter<'_, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Iter { iter : Default :: default () } } }
};
}
