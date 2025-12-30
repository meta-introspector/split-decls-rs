// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_btree_mapIterMut {
() => {
// Module: crate::collections::btree::map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the entries of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: BTreeMap::iter_mut"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IterMut < 'a , K : 'a , V : 'a > { range : LazyLeafRange < marker :: ValMut < 'a > , K , V > , length : usize , _marker : PhantomData < & 'a mut (K , V) > , }
};
}
