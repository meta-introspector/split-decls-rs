// Generated macro for impl_170 (impl)
macro_rules! Depcrate_rayon_mapimpl_170 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , K , V , S > IntoParallelIterator for & 'a IndexMap < K , V , S > where K : Sync , V : Sync , { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : self . as_entries () , } } }
};
}
