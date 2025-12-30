// Generated macro for ParSymmetricDifference (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setParSymmetricDifference {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"ParSymmetricDifference"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to elements in the symmetric"] # [doc = " difference of sets."] # [doc = ""] # [doc = " This iterator is created by the [`par_symmetric_difference`] method on"] # [doc = " [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_symmetric_difference`]: /hashbrown/struct.HashSet.html#method.par_symmetric_difference"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] pub struct ParSymmetricDifference < 'a , T , S , A : Allocator = Global > { a : & 'a HashSet < T , S , A > , b : & 'a HashSet < T , S , A > , }
};
}
