// Generated macro for Values (struct)
macro_rules! Depcrate_index_mapValues {
() => {
// Module: crate::index_map
// Provides: {"Values"}
// Dependencies: {}
# [doc = " An iterator over the values of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`values`](IndexMap::values) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct Values < 'a , K , V > { iter : slice :: Iter < 'a , Bucket < K , V > > , }
};
}
