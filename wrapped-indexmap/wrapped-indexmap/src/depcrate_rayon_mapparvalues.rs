// Generated macro for ParValues (struct)
macro_rules! Depcrate_rayon_mapParValues {
() => {
// Module: crate::rayon::map
// Provides: {"ParValues"}
// Dependencies: {}
# [doc = " A parallel iterator over the values of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_values`] method."] # [doc = " See its documentation for more."] pub struct ParValues < 'a , K , V > { entries : & 'a [Bucket < K , V >] , }
};
}
