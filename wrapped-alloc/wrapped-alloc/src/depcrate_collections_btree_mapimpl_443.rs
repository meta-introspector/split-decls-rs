// Generated macro for impl_443 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_443 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_443"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V > Default for Keys < '_ , K , V > { # [doc = " Creates an empty `btree_map::Keys`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::Keys<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Keys { inner : Default :: default () } } }
};
}
