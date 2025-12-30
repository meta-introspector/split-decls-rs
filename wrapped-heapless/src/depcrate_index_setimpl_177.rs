// Generated macro for impl_177 (impl)
macro_rules! Depcrate_index_setimpl_177 {
() => {
// Module: crate::index_set
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a , T , S , const N : usize > IntoIterator for & 'a IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
