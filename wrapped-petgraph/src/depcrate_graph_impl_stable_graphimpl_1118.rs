// Generated macro for impl_1118 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1118 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1118"}
// Dependencies: {}
impl < 'a , N , Ix > Iterator for NodeReferences < 'a , N , Ix > where Ix : IndexType , { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , node) | node . weight . as_ref () . map (move | w | (node_index (i) , w))) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , hi) = self . iter . size_hint () ; (0 , hi) } }
};
}
