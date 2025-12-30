// Generated macro for Difference (struct)
macro_rules! Depcrate_index_setDifference {
() => {
// Module: crate::index_set
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " An iterator over the difference of two `IndexSet`s."] # [doc = ""] # [doc = " This is created by the [`IndexSet::difference`] method."] pub struct Difference < 'a , T , S , const N : usize > where S : BuildHasher , T : Eq + Hash , { iter : Iter < 'a , T > , other : & 'a IndexSet < T , S , N > , }
};
}
