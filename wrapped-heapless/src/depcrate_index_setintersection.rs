// Generated macro for Intersection (struct)
macro_rules! Depcrate_index_setIntersection {
() => {
// Module: crate::index_set
// Provides: {"Intersection"}
// Dependencies: {}
# [doc = " An iterator over the intersection of two `IndexSet`s."] # [doc = ""] # [doc = " This is created by the [`IndexSet::intersection`] method."] pub struct Intersection < 'a , T , S , const N : usize > where S : BuildHasher , T : Eq + Hash , { iter : Iter < 'a , T > , other : & 'a IndexSet < T , S , N > , }
};
}
