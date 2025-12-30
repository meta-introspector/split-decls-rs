// Generated macro for impl_152 (impl)
macro_rules! Depcrate_linked_hash_setimpl_152 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a , T , S > Extend < & 'a T > for LinkedHashSet < T , S > where T : 'a + Eq + Hash + Copy , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } }
};
}
