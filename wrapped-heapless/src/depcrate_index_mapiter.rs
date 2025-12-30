// Generated macro for Iter (struct)
macro_rules! Depcrate_index_mapIter {
() => {
// Module: crate::index_map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`](IndexMap::iter) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct Iter < 'a , K , V > { iter : slice :: Iter < 'a , Bucket < K , V > > , }
};
}
