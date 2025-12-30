// Generated macro for ParIter (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setParIter {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"ParIter"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to elements in a set."] # [doc = ""] # [doc = " This iterator is created by the [`par_iter`] method on [`HashSet`]"] # [doc = " (provided by the [`IntoParallelRefIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_iter`]: /hashbrown/struct.HashSet.html#method.par_iter"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] # [doc = " [`IntoParallelRefIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefIterator.html"] pub struct ParIter < 'a , T > { inner : map :: ParKeys < 'a , T , () > , }
};
}
