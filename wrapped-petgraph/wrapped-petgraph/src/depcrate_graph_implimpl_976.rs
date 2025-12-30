// Generated macro for impl_976 (impl)
macro_rules! Depcrate_graph_implimpl_976 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_976"}
// Dependencies: {}
impl < 'a , N , Ix > Iterator for NodeWeights < 'a , N , Ix > where Ix : IndexType , { type Item = & 'a N ; fn next (& mut self) -> Option < & 'a N > { self . nodes . next () . map (| node | & node . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . nodes . size_hint () } }
};
}
