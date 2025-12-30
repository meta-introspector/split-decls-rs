// Generated macro for impl_480 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_480 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_480"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : Ord , V > FromIterator < (K , V) > for BTreeMap < K , V > { # [doc = " Constructs a `BTreeMap<K, V>` from an iterator of key-value pairs."] # [doc = ""] # [doc = " If the iterator produces any pairs with equal keys,"] # [doc = " all but one of the corresponding values will be dropped."] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> BTreeMap < K , V > { let mut inputs : Vec < _ > = iter . into_iter () . collect () ; if inputs . is_empty () { return BTreeMap :: new () ; } inputs . sort_by (| a , b | a . 0 . cmp (& b . 0)) ; BTreeMap :: bulk_build_from_sorted_iter (inputs , Global) } }
};
}
