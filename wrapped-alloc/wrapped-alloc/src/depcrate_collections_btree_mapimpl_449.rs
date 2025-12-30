// Generated macro for impl_449 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_449 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_449"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V > Default for Values < '_ , K , V > { # [doc = " Creates an empty `btree_map::Values`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::Values<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Values { inner : Default :: default () } } }
};
}
