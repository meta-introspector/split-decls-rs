// Generated macro for impl_151 (impl)
macro_rules! Depcrate_linked_hash_setimpl_151 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_151"}
// Dependencies: {}
impl < T , S > Extend < T > for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . map . extend (iter . into_iter () . map (| k | (k , ()))) ; } }
};
}
