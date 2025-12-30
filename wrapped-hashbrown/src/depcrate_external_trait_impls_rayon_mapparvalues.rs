// Generated macro for ParValues (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParValues {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParValues"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to values in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_values`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_values`]: /hashbrown/struct.HashMap.html#method.par_values"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParValues < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
};
}
