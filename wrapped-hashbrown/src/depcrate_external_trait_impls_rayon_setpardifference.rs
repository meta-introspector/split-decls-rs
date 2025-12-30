// Generated macro for ParDifference (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setParDifference {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"ParDifference"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to elements in the difference of"] # [doc = " sets."] # [doc = ""] # [doc = " This iterator is created by the [`par_difference`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_difference`]: /hashbrown/struct.HashSet.html#method.par_difference"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] pub struct ParDifference < 'a , T , S , A : Allocator = Global > { a : & 'a HashSet < T , S , A > , b : & 'a HashSet < T , S , A > , }
};
}
