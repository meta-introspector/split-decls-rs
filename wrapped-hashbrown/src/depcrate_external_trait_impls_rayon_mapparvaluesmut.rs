// Generated macro for ParValuesMut (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParValuesMut {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParValuesMut"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable references to values in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_values_mut`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_values_mut`]: /hashbrown/struct.HashMap.html#method.par_values_mut"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParValuesMut < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a mut V) > , }
};
}
