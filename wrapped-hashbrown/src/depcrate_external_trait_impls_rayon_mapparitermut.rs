// Generated macro for ParIterMut (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParIterMut {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParIterMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable references to entries in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_iter_mut`] method on [`HashMap`]"] # [doc = " (provided by the [`IntoParallelRefMutIterator`] trait)."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_iter_mut`]: /hashbrown/struct.HashMap.html#method.par_iter_mut"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] # [doc = " [`IntoParallelRefMutIterator`]: https://docs.rs/rayon/1.0/rayon/iter/trait.IntoParallelRefMutIterator.html"] pub struct ParIterMut < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a mut V) > , }
};
}
