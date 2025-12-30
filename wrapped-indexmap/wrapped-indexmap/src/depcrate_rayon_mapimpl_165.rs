// Generated macro for impl_165 (impl)
macro_rules! Depcrate_rayon_mapimpl_165 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_165"}
// Dependencies: {}
impl < K , V > IntoParallelIterator for Box < Slice < K , V > > where K : Send , V : Send , { type Item = (K , V) ; type Iter = IntoParIter < K , V > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
};
}
