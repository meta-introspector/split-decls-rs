// Generated macro for impl_224 (impl)
macro_rules! Depcrate_rayon_setimpl_224 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_224"}
// Dependencies: {}
impl < T , S > IntoParallelIterator for IndexSet < T , S > where T : Send , { type Item = T ; type Iter = IntoParIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
};
}
