// Generated macro for impl_164 (impl)
macro_rules! Depcrate_rayon_mapimpl_164 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_164"}
// Dependencies: {}
impl < K , V , S > IntoParallelIterator for IndexMap < K , V , S > where K : Send , V : Send , { type Item = (K , V) ; type Iter = IntoParIter < K , V > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
};
}
