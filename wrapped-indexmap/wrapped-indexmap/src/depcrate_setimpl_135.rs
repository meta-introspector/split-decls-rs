// Generated macro for impl_135 (impl)
macro_rules! Depcrate_setimpl_135 {
() => {
// Module: crate::set
// Provides: {"impl_135"}
// Dependencies: {}
impl < T , S > FromIterator < T > for IndexSet < T , S > where T : Hash + Eq , S : BuildHasher + Default , { fn from_iter < I : IntoIterator < Item = T > > (iterable : I) -> Self { let iter = iterable . into_iter () . map (| x | (x , ())) ; IndexSet { map : IndexMap :: from_iter (iter) , } } }
};
}
