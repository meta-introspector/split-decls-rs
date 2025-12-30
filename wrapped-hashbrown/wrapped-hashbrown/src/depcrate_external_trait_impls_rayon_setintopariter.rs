// Generated macro for IntoParIter (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setIntoParIter {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"IntoParIter"}
// Dependencies: {}
# [doc = " Parallel iterator over elements of a consumed set."] # [doc = ""] # [doc = " This iterator is created by the [`into_par_iter`] method on [`HashSet`]"] # [doc = " (provided by the [`IntoParallelIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`into_par_iter`]: /hashbrown/struct.HashSet.html#method.into_par_iter"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] # [doc = " [`IntoParallelIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelIterator.html"] pub struct IntoParIter < T , A : Allocator = Global > { inner : map :: IntoParIter < T , () , A > , }
};
}
