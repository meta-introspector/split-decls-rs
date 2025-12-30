// Generated macro for ParIntersection (struct)
macro_rules! Depcrate_external_trait_impls_rayon_setParIntersection {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"ParIntersection"}
// Dependencies: {}
# [doc = " Parallel iterator over shared references to elements in the intersection of"] # [doc = " sets."] # [doc = ""] # [doc = " This iterator is created by the [`par_intersection`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_intersection`]: /hashbrown/struct.HashSet.html#method.par_intersection"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] pub struct ParIntersection < 'a , T , S , A : Allocator = Global > { a : & 'a HashSet < T , S , A > , b : & 'a HashSet < T , S , A > , }
};
}
