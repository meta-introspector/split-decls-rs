// Generated macro for impl_178 (impl)
macro_rules! Depcrate_rayon_mapimpl_178 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , K , V > IntoParallelIterator for & 'a mut Slice < K , V > where K : Sync + Send , V : Send , { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIterMut { entries : & mut self . entries , } } }
};
}
