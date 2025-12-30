// Generated macro for impl_260 (impl)
macro_rules! Depcrate_rayon_setimpl_260 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_260"}
// Dependencies: {}
impl < T , S > ParallelExtend < T > for IndexSet < T , S > where T : Eq + Hash + Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = T > , { for vec in collect (iter) { self . extend (vec) ; } } }
};
}
