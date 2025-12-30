// Generated macro for impl_100 (impl)
macro_rules! Depcrate_ordered_setimpl_100 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
