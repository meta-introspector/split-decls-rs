// Generated macro for impl_468 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_468 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_468"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V , A > Default for IntoKeys < K , V , A > where A : Allocator + Default + Clone , { # [doc = " Creates an empty `btree_map::IntoKeys`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::IntoKeys<u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IntoKeys { inner : Default :: default () } } }
};
}
