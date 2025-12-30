// Generated macro for impl_171 (impl)
macro_rules! Depcrate_rayon_mapimpl_171 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'a , K , V > IntoParallelIterator for & 'a Slice < K , V > where K : Sync , V : Sync , { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : & self . entries , } } }
};
}
