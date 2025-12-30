// Generated macro for impl_762 (impl)
macro_rules! Depcrate_csrimpl_762 {
() => {
// Module: crate::csr
// Provides: {"impl_762"}
// Dependencies: {}
impl < Ix > Iterator for Neighbors < '_ , Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . cloned () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
