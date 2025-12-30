// Generated macro for impl_231 (impl)
macro_rules! Depcrate_rayon_setimpl_231 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a , T > IntoParallelIterator for & 'a Slice < T > where T : Sync , { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : & self . entries , } } }
};
}
