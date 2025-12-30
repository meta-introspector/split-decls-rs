// Generated macro for impl_185 (impl)
macro_rules! Depcrate_linked_hash_setimpl_185 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'a , T , S > Iterator for SymmetricDifference < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
