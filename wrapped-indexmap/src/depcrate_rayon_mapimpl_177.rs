// Generated macro for impl_177 (impl)
macro_rules! Depcrate_rayon_mapimpl_177 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a , K , V , S > IntoParallelIterator for & 'a mut IndexMap < K , V , S > where K : Sync + Send , V : Send , { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIterMut { entries : self . as_entries_mut () , } } }
};
}
