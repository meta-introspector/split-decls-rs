// Generated macro for impl_457 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_457 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_457"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V > Default for Range < '_ , K , V > { # [doc = " Creates an empty `btree_map::Range`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::Range<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.count(), 0);"] # [doc = " ```"] fn default () -> Self { Range { inner : Default :: default () } } }
};
}
