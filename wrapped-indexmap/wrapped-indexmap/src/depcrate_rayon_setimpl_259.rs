// Generated macro for impl_259 (impl)
macro_rules! Depcrate_rayon_setimpl_259 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_259"}
// Dependencies: {}
impl < T , S > FromParallelIterator < T > for IndexSet < T , S > where T : Eq + Hash + Send , S : BuildHasher + Default + Send , { fn from_par_iter < I > (iter : I) -> Self where I : IntoParallelIterator < Item = T > , { let list = collect (iter) ; let len = list . iter () . map (Vec :: len) . sum () ; let mut set = Self :: with_capacity_and_hasher (len , S :: default ()) ; for vec in list { set . extend (vec) ; } set } }
};
}
