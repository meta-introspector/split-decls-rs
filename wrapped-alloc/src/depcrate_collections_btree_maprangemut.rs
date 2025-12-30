// Generated macro for RangeMut (struct)
macro_rules! Depcrate_collections_btree_mapRangeMut {
() => {
// Module: crate::collections::btree::map
// Provides: {"RangeMut"}
// Dependencies: {}
# [doc = " A mutable iterator over a sub-range of entries in a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`range_mut`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`range_mut`]: BTreeMap::range_mut"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "btree_range" , since = "1.17.0")] pub struct RangeMut < 'a , K : 'a , V : 'a > { inner : LeafRange < marker :: ValMut < 'a > , K , V > , _marker : PhantomData < & 'a mut (K , V) > , }
};
}
