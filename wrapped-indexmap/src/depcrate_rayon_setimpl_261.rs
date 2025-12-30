// Generated macro for impl_261 (impl)
macro_rules! Depcrate_rayon_setimpl_261 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'a , T : 'a , S > ParallelExtend < & 'a T > for IndexSet < T , S > where T : Copy + Eq + Hash + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = & 'a T > , { for vec in collect (iter) { self . extend (vec) ; } } }
};
}
