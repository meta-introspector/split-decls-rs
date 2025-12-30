// Generated macro for impl_195 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_195 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_195"}
// Dependencies: {}
# [doc = " Extend a hash map with copied items from a parallel iterator."] impl < 'a , K , V , S , A > ParallelExtend < (& 'a K , & 'a V) > for HashMap < K , V , S , A > where K : Copy + Eq + Hash + Sync , V : Copy + Sync , S : BuildHasher , A : Allocator , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { extend (self , par_iter) ; } }
};
}
