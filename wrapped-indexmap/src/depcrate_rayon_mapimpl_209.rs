// Generated macro for impl_209 (impl)
macro_rules! Depcrate_rayon_mapimpl_209 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , S > ParallelExtend < (& 'a K , & 'a V) > for IndexMap < K , V , S > where K : Copy + Eq + Hash + Send + Sync , V : Copy + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { for vec in collect (iter) { self . extend (vec) ; } } }
};
}
