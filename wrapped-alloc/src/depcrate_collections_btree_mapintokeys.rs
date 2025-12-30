// Generated macro for IntoKeys (struct)
macro_rules! Depcrate_collections_btree_mapIntoKeys {
() => {
// Module: crate::collections::btree::map
// Provides: {"IntoKeys"}
// Dependencies: {}
# [doc = " An owning iterator over the keys of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_keys`] method on [`BTreeMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_keys`]: BTreeMap::into_keys"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "map_into_keys_values" , since = "1.54.0")] pub struct IntoKeys < K , V , A : Allocator + Clone = Global > { inner : IntoIter < K , V , A > , }
};
}
