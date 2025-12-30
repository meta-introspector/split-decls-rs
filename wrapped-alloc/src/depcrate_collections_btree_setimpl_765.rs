// Generated macro for impl_765 (impl)
macro_rules! Depcrate_collections_btree_setimpl_765 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_765"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T > Default for Range < '_ , T > { # [doc = " Creates an empty `btree_set::Range`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_set;"] # [doc = " let iter: btree_set::Range<'_, u8> = Default::default();"] # [doc = " assert_eq!(iter.count(), 0);"] # [doc = " ```"] fn default () -> Self { Range { iter : Default :: default () } } }
};
}
