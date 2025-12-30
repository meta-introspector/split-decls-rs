// Generated macro for Keys (struct)
macro_rules! Depcrate_index_mapKeys {
() => {
// Module: crate::index_map
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " An iterator over the keys of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`keys`](IndexMap::keys) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct Keys < 'a , K , V > { iter : slice :: Iter < 'a , Bucket < K , V > > , }
};
}
