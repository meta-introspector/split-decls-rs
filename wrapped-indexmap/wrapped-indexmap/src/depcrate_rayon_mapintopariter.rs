// Generated macro for IntoParIter (struct)
macro_rules! Depcrate_rayon_mapIntoParIter {
() => {
// Module: crate::rayon::map
// Provides: {"IntoParIter"}
// Dependencies: {}
# [doc = " A parallel owning iterator over the entries of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::into_par_iter`] method"] # [doc = " (provided by rayon's [`IntoParallelIterator`] trait). See its documentation for more."] pub struct IntoParIter < K , V > { entries : Vec < Bucket < K , V > > , }
};
}
