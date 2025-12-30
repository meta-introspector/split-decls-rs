// Generated macro for ParIntersection (struct)
macro_rules! Depcrate_rayon_setParIntersection {
() => {
// Module: crate::rayon::set
// Provides: {"ParIntersection"}
// Dependencies: {}
# [doc = " A parallel iterator producing elements in the intersection of [`IndexSet`]s."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_intersection`] method."] # [doc = " See its documentation for more."] pub struct ParIntersection < 'a , T , S1 , S2 > { set1 : & 'a IndexSet < T , S1 > , set2 : & 'a IndexSet < T , S2 > , }
};
}
