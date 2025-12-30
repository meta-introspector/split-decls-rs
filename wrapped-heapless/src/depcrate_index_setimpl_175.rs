// Generated macro for impl_175 (impl)
macro_rules! Depcrate_index_setimpl_175 {
() => {
// Module: crate::index_set
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'a , T , S , const N : usize > Extend < & 'a T > for IndexSet < T , S , N > where T : 'a + Eq + Hash + Copy , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iterable . into_iter () . cloned ()) ; } }
};
}
