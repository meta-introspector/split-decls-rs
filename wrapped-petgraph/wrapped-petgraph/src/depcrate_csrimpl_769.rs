// Generated macro for impl_769 (impl)
macro_rules! Depcrate_csrimpl_769 {
() => {
// Module: crate::csr
// Provides: {"impl_769"}
// Dependencies: {}
impl < Ix > Iterator for NodeIdentifiers < Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (Ix :: new) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
};
}
