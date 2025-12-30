// Generated macro for impl_35 (impl)
macro_rules! Depcrate_iter_setimpl_35 {
() => {
// Module: crate::iter_set
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + 'a > Iterator for Iter < 'a , K > { type Item = RefMulti < 'a , K > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (RefMulti :: new) } }
};
}
