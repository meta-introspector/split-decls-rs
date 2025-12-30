// Generated macro for Range (struct)
macro_rules! Depcrate_collections_btree_mapRange {
() => {
// Module: crate::collections::btree::map
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a sub-range of entries in a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`range`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`range`]: BTreeMap::range"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "btree_range" , since = "1.17.0")] pub struct Range < 'a , K : 'a , V : 'a > { inner : LeafRange < marker :: Immut < 'a > , K , V > , }
};
}
