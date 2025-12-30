// Generated macro for IntoValues (struct)
macro_rules! Depcrate_collections_btree_mapIntoValues {
() => {
// Module: crate::collections::btree::map
// Provides: {"IntoValues"}
// Dependencies: {}
# [doc = " An owning iterator over the values of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_values`] method on [`BTreeMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_values`]: BTreeMap::into_values"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "map_into_keys_values" , since = "1.54.0")] pub struct IntoValues < K , V , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { inner : IntoIter < K , V , A > , }
};
}
