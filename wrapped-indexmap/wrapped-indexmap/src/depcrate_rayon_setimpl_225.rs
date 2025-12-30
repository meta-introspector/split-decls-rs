// Generated macro for impl_225 (impl)
macro_rules! Depcrate_rayon_setimpl_225 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_225"}
// Dependencies: {}
impl < T > IntoParallelIterator for Box < Slice < T > > where T : Send , { type Item = T ; type Iter = IntoParIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
};
}
