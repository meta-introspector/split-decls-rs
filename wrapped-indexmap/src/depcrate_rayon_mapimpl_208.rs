// Generated macro for impl_208 (impl)
macro_rules! Depcrate_rayon_mapimpl_208 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_208"}
// Dependencies: {}
impl < K , V , S > ParallelExtend < (K , V) > for IndexMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = (K , V) > , { for vec in collect (iter) { self . extend (vec) ; } } }
};
}
