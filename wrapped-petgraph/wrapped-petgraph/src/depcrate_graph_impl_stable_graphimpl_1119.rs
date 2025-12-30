// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1119 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1119"}
// Dependencies: {}
impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | node . weight . as_ref () . map (move | w | (node_index (i) , w))) } }
};
}
