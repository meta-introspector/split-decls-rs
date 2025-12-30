// Generated macro for impl_168 (impl)
macro_rules! Depcrate_linked_hash_setimpl_168 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a , K > Iterator for Iter < 'a , K > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
