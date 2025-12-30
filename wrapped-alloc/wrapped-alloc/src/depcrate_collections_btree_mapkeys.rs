// Generated macro for Keys (struct)
macro_rules! Depcrate_collections_btree_mapKeys {
() => {
// Module: crate::collections::btree::map
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over the keys of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`keys`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`keys`]: BTreeMap::keys"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Keys < 'a , K , V > { inner : Iter < 'a , K , V > , }
};
}
