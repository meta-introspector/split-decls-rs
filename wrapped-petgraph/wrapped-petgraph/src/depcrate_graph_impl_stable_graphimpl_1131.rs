// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1131 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1131"}
// Dependencies: {}
impl < 'a , E , Ix > Iterator for EdgeReferences < 'a , E , Ix > where Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . ex_find_map (| (i , edge) | { edge . weight . as_ref () . map (move | weight | EdgeReference { index : edge_index (i) , node : edge . node , weight , }) }) } }
};
}
