// Generated macro for ParKeys (struct)
macro_rules! Depcrate_rayon_mapParKeys {
() => {
// Module: crate::rayon::map
// Provides: {"ParKeys"}
// Dependencies: {}
# [doc = " A parallel iterator over the keys of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_keys`] method."] # [doc = " See its documentation for more."] pub struct ParKeys < 'a , K , V > { entries : & 'a [Bucket < K , V >] , }
};
}
