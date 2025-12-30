// Generated macro for impl_1019 (impl)
macro_rules! Depcrate_graph_implimpl_1019 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1019"}
// Dependencies: {}
impl < 'a , N , Ix > Iterator for NodeReferences < 'a , N , Ix > where Ix : IndexType , { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , node) | (node_index (i) , & node . weight)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
