// Generated macro for impl_394 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_394 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_394"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < 'a , K : 'a , V : 'a > Default for Iter < 'a , K , V > { # [doc = " Creates an empty `btree_map::Iter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::Iter<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Iter { range : Default :: default () , length : 0 } } }
};
}
