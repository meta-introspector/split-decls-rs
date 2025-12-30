// Generated macro for Values (struct)
macro_rules! Depcrate_collections_btree_mapValues {
() => {
// Module: crate::collections::btree::map
// Provides: {"Values"}
// Dependencies: {}
# [doc = " An iterator over the values of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`values`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values`]: BTreeMap::values"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Values < 'a , K , V > { inner : Iter < 'a , K , V > , }
};
}
