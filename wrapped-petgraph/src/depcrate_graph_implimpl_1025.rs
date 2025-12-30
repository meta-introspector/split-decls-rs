// Generated macro for impl_1025 (impl)
macro_rules! Depcrate_graph_implimpl_1025 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1025"}
// Dependencies: {}
impl < 'a , E , Ix > Iterator for EdgeReferences < 'a , E , Ix > where Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , edge) | EdgeReference { index : edge_index (i) , node : edge . node , weight : & edge . weight , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
