// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < I : Iterator , const N : usize > Iterator for Splice < '_ , I , N > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
};
}
