// Generated macro for impl_174 (impl)
macro_rules! Depcrate_index_setimpl_174 {
() => {
// Module: crate::index_set
// Provides: {"impl_174"}
// Dependencies: {}
impl < T , S , const N : usize > Extend < T > for IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = T > , { self . map . extend (iterable . into_iter () . map (| k | (k , ()))) ; } }
};
}
