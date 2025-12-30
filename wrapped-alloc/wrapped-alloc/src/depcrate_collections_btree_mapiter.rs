// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_btree_mapIter {
() => {
// Module: crate::collections::btree::map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: BTreeMap::iter"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Iter < 'a , K : 'a , V : 'a > { range : LazyLeafRange < marker :: Immut < 'a > , K , V > , length : usize , }
};
}
