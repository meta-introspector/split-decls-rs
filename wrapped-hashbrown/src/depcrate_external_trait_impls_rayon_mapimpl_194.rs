// Generated macro for impl_194 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_194 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_194"}
// Dependencies: {}
# [doc = " Extend a hash map with items from a parallel iterator."] impl < K , V , S , A > ParallelExtend < (K , V) > for HashMap < K , V , S , A > where K : Eq + Hash + Send , V : Send , S : BuildHasher , A : Allocator , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend (self , par_iter) ; } }
};
}
