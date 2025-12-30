// Generated macro for impl_978 (impl)
macro_rules! Depcrate_graph_implimpl_978 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_978"}
// Dependencies: {}
impl < 'a , N , Ix > Iterator for NodeWeightsMut < 'a , N , Ix > where Ix : IndexType , { type Item = & 'a mut N ; fn next (& mut self) -> Option < & 'a mut N > { self . nodes . next () . map (| node | & mut node . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . nodes . size_hint () } }
};
}
