// Generated macro for IterMut (struct)
macro_rules! Depcrate_index_mapIterMut {
() => {
// Module: crate::index_map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the items of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`](IndexMap::iter_mut) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct IterMut < 'a , K , V > { iter : slice :: IterMut < 'a , Bucket < K , V > > , }
};
}
