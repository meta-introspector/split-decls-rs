// Generated macro for ParDrain (struct)
macro_rules! Depcrate_external_trait_impls_rayon_mapParDrain {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"ParDrain"}
// Dependencies: {}
# [doc = " Parallel draining iterator over entries of a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_drain`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_drain`]: /hashbrown/struct.HashMap.html#method.par_drain"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParDrain < 'a , K , V , A : Allocator = Global > { inner : RawParDrain < 'a , (K , V) , A > , }
};
}
