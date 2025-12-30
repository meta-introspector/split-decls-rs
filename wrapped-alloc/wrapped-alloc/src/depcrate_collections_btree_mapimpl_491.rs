// Generated macro for impl_491 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_491 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_491"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < K : Ord , V , const N : usize > From < [(K , V) ; N] > for BTreeMap < K , V > { # [doc = " Converts a `[(K, V); N]` into a `BTreeMap<K, V>`."] # [doc = ""] # [doc = " If any entries in the array have equal keys,"] # [doc = " all but one of the corresponding values will be dropped."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeMap;"] # [doc = ""] # [doc = " let map1 = BTreeMap::from([(1, 2), (3, 4)]);"] # [doc = " let map2: BTreeMap<_, _> = [(1, 2), (3, 4)].into();"] # [doc = " assert_eq!(map1, map2);"] # [doc = " ```"] fn from (mut arr : [(K , V) ; N]) -> Self { if N == 0 { return BTreeMap :: new () ; } arr . sort_by (| a , b | a . 0 . cmp (& b . 0)) ; BTreeMap :: bulk_build_from_sorted_iter (arr , Global) } }
};
}
