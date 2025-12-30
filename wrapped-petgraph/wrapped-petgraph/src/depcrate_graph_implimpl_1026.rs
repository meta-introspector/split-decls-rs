// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_graph_implimpl_1026 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1026"}
// Dependencies: {}
impl < E , Ix > DoubleEndedIterator for EdgeReferences < '_ , E , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , edge) | EdgeReference { index : edge_index (i) , node : edge . node , weight : & edge . weight , }) } }
};
}
