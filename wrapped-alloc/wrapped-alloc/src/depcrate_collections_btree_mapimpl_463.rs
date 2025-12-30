// Generated macro for impl_463 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_463 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_463"}
// Dependencies: {}
# [stable (feature = "default_iters_sequel" , since = "1.82.0")] impl < K , V > Default for ValuesMut < '_ , K , V > { # [doc = " Creates an empty `btree_map::ValuesMut`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::ValuesMut<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.count(), 0);"] # [doc = " ```"] fn default () -> Self { ValuesMut { inner : Default :: default () } } }
};
}
