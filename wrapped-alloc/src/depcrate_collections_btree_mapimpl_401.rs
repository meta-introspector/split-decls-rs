// Generated macro for impl_401 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_401 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_401"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < K , V , A > Default for IntoIter < K , V , A > where A : Allocator + Default + Clone , { # [doc = " Creates an empty `btree_map::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::IntoIter<u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IntoIter { range : Default :: default () , length : 0 , alloc : Default :: default () } } }
};
}
