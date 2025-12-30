// Generated macro for impl_150 (impl)
macro_rules! Depcrate_linked_hash_setimpl_150 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_150"}
// Dependencies: {}
impl < T , S > FromIterator < T > for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher + Default , { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> LinkedHashSet < T , S > { let mut set = LinkedHashSet :: with_hasher (Default :: default ()) ; set . extend (iter) ; set } }
};
}
