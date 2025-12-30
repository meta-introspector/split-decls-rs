// Generated macro for ParDrain (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setParDrain {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"ParDrain"}
// Dependencies: {}
# [doc = " Parallel draining iterator over entries of a set."] # [doc = ""] # [doc = " This iterator is created by the [`par_drain`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_drain`]: /hashbrown/struct.HashSet.html#method.par_drain"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] pub struct ParDrain < 'a , T , A : Allocator = Global > { inner : map :: ParDrain < 'a , T , () , A > , }
};
}
