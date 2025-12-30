// Generated macro for IntoParIter (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapIntoParIter {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"IntoParIter"}
// Dependencies: {}
# [doc = " Parallel iterator over entries of a consumed map."] # [doc = ""] # [doc = " This iterator is created by the [`into_par_iter`] method on [`HashMap`]"] # [doc = " (provided by the [`IntoParallelIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_par_iter`]: /hashbrown/struct.HashMap.html#method.into_par_iter"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] # [doc = " [`IntoParallelIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelIterator.html"] pub struct IntoParIter < K , V , A : Allocator = Global > { inner : RawIntoParIter < (K , V) , A > , }
};
}
