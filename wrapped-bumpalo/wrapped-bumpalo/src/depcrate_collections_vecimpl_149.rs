// Generated macro for impl_149 (impl)
macro_rules! Depcrate_collections_vecimpl_149 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a , 'bump , I : Iterator > Iterator for Splice < 'a , 'bump , I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
};
}
