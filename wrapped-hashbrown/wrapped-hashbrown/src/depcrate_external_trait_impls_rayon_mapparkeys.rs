// Generated macro for ParKeys (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParKeys {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParKeys"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to keys in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_keys`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_keys`]: /hashbrown/struct.HashMap.html#method.par_keys"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParKeys < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
};
}
