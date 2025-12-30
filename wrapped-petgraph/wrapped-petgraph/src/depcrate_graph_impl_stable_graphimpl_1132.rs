// Generated macro for impl_1132 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1132 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1132"}
// Dependencies: {}
impl < E , Ix > DoubleEndedIterator for EdgeReferences < '_ , E , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , edge) | { edge . weight . as_ref () . map (move | weight | EdgeReference { index : edge_index (i) , node : edge . node , weight , }) }) } }
};
}
