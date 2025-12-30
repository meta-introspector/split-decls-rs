// Generated macro for impl_137 (impl)
macro_rules! Depcrate_setimpl_137 {
() => {
// Module: crate::set
// Provides: {"impl_137"}
// Dependencies: {}
impl < T , S > Extend < T > for IndexSet < T , S > where T : Hash + Eq , S : BuildHasher , { fn extend < I : IntoIterator < Item = T > > (& mut self , iterable : I) { let iter = iterable . into_iter () . map (| x | (x , ())) ; self . map . extend (iter) ; } }
};
}
