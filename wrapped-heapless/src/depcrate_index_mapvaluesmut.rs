// Generated macro for ValuesMut (struct)
macro_rules! Depcrate_index_mapValuesMut {
() => {
// Module: crate::index_map
// Provides: {"ValuesMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the values of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`values_mut`](IndexMap::values_mut) method on [`IndexMap`]. See"] # [doc = " its documentation for more."] pub struct ValuesMut < 'a , K , V > { iter : slice :: IterMut < 'a , Bucket < K , V > > , }
};
}
