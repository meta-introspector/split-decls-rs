// Generated macro for ParDifference (struct)
macro_rules! Depcrate_rayon_setParDifference {
() => {
// Module: crate::rayon::set
// Provides: {"ParDifference"}
// Dependencies: {}
# [doc = " A parallel iterator producing elements in the difference of [`IndexSet`]s."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_difference`] method."] # [doc = " See its documentation for more."] pub struct ParDifference < 'a , T , S1 , S2 > { set1 : & 'a IndexSet < T , S1 > , set2 : & 'a IndexSet < T , S2 > , }
};
}
