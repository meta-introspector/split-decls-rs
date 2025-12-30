// Generated macro for ParIter (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParIter {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParIter"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to entries in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_iter`] method on [`HashMap`]"] # [doc = " (provided by the [`IntoParallelRefIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_iter`]: /hashbrown/struct.HashMap.html#method.par_iter"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] # [doc = " [`IntoParallelRefIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefIterator.html"] pub struct ParIter < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
};
}
