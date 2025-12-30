// Generated macro for IntoParIter (struct)
macro_rules! Depcrate_rayon_setIntoParIter {
() => {
// Module: crate::rayon::set
// Provides: {"IntoParIter"}
// Dependencies: {}
# [doc = " A parallel owning iterator over the items of an [`IndexSet`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::into_par_iter`] method"] # [doc = " (provided by rayon's [`IntoParallelIterator`] trait). See its documentation for more."] pub struct IntoParIter < T > { entries : Vec < Bucket < T > > , }
};
}
