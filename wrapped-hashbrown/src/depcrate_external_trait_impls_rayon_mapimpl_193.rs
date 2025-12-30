// Generated macro for impl_193 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_193 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_193"}
// Dependencies: {}
# [doc = " Collect (key, value) pairs from a parallel iterator into a"] # [doc = " hashmap. If multiple pairs correspond to the same key, then the"] # [doc = " ones produced earlier in the parallel iterator will be"] # [doc = " overwritten, just as with a sequential iterator."] impl < K , V , S > FromParallelIterator < (K , V) > for HashMap < K , V , S , Global > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Default , { fn from_par_iter < P > (par_iter : P) -> Self where P : IntoParallelIterator < Item = (K , V) > , { let mut map = HashMap :: default () ; map . par_extend (par_iter) ; map } }
};
}
