// Generated macro for ParIter (struct)
macro_rules! Depcrate_rayon_mapParIter {
() => {
// Module: crate::rayon::map
// Provides: {"ParIter"}
// Dependencies: {}
# [doc = " A parallel iterator over the entries of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_iter`] method"] # [doc = " (provided by rayon's [`IntoParallelRefIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`IndexMap::par_iter`]: ../struct.IndexMap.html#method.par_iter"] pub struct ParIter < 'a , K , V > { entries : & 'a [Bucket < K , V >] , }
};
}
