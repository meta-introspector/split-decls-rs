// Generated macro for ParSymmetricDifference (struct)
macro_rules! Depcrate_rayon_setParSymmetricDifference {
() => {
// Module: crate::rayon::set
// Provides: {"ParSymmetricDifference"}
// Dependencies: {}
# [doc = " A parallel iterator producing elements in the symmetric difference of [`IndexSet`]s."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_symmetric_difference`] method."] # [doc = " See its documentation for more."] pub struct ParSymmetricDifference < 'a , T , S1 , S2 > { set1 : & 'a IndexSet < T , S1 > , set2 : & 'a IndexSet < T , S2 > , }
};
}
