// Generated macro for impl_230 (impl)
macro_rules! Depcrate_rayon_setimpl_230 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a , T , S > IntoParallelIterator for & 'a IndexSet < T , S > where T : Sync , { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : self . as_entries () , } } }
};
}
