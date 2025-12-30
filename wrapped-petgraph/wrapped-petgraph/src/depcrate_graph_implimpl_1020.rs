// Generated macro for impl_1020 (impl)
macro_rules! Depcrate_graph_implimpl_1020 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1020"}
// Dependencies: {}
impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , node) | (node_index (i) , & node . weight)) } }
};
}
