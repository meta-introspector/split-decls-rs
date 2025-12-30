// Generated macro for impl_145 (impl)
macro_rules! Depcrate_linked_hash_setimpl_145 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_145"}
// Dependencies: {}
impl < T : Hash + Eq + Clone , S : BuildHasher + Clone > Clone for LinkedHashSet < T , S > { # [inline] fn clone (& self) -> Self { let map = self . map . clone () ; Self { map } } }
};
}
