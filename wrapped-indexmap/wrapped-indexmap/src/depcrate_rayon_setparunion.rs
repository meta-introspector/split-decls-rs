// Generated macro for ParUnion (struct)
macro_rules! Depcrate_rayon_setParUnion {
() => {
// Module: crate::rayon::set
// Provides: {"ParUnion"}
// Dependencies: {}
# [doc = " A parallel iterator producing elements in the union of [`IndexSet`]s."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_union`] method."] # [doc = " See its documentation for more."] pub struct ParUnion < 'a , T , S1 , S2 > { set1 : & 'a IndexSet < T , S1 > , set2 : & 'a IndexSet < T , S2 > , }
};
}
