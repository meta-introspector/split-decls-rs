// Generated macro for ParValuesMut (struct)
macro_rules! Depcrate_rayon_mapParValuesMut {
() => {
// Module: crate::rayon::map
// Provides: {"ParValuesMut"}
// Dependencies: {}
# [doc = " A parallel mutable iterator over the values of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_values_mut`] method."] # [doc = " See its documentation for more."] pub struct ParValuesMut < 'a , K , V > { entries : & 'a mut [Bucket < K , V >] , }
};
}
