// Generated macro for impl_207 (impl)
macro_rules! Depcrate_rayon_mapimpl_207 {
() => {
// Module: crate::rayon::map
// Provides: {"impl_207"}
// Dependencies: {}
impl < K , V , S > FromParallelIterator < (K , V) > for IndexMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Default + Send , { fn from_par_iter < I > (iter : I) -> Self where I : IntoParallelIterator < Item = (K , V) > , { let list = collect (iter) ; let len = list . iter () . map (Vec :: len) . sum () ; let mut map = Self :: with_capacity_and_hasher (len , S :: default ()) ; for vec in list { map . extend (vec) ; } map } }
};
}
