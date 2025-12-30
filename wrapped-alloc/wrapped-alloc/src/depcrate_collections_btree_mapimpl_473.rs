// Generated macro for impl_473 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_473 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_473"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V , A > Default for IntoValues < K , V , A > where A : Allocator + Default + Clone , { # [doc = " Creates an empty `btree_map::IntoValues`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::IntoValues<u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IntoValues { inner : Default :: default () } } }
};
}
