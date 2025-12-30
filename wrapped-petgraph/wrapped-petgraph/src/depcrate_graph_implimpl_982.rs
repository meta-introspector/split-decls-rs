// Generated macro for impl_982 (impl)
macro_rules! Depcrate_graph_implimpl_982 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_982"}
// Dependencies: {}
impl < 'a , E , Ix > Iterator for EdgeWeightsMut < 'a , E , Ix > where Ix : IndexType , { type Item = & 'a mut E ; fn next (& mut self) -> Option < & 'a mut E > { self . edges . next () . map (| edge | & mut edge . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . edges . size_hint () } }
};
}
