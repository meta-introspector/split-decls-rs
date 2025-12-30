// Generated macro for impl_980 (impl)
macro_rules! Depcrate_graph_implimpl_980 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_980"}
// Dependencies: {}
impl < 'a , E , Ix > Iterator for EdgeWeights < 'a , E , Ix > where Ix : IndexType , { type Item = & 'a E ; fn next (& mut self) -> Option < & 'a E > { self . edges . next () . map (| edge | & edge . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . edges . size_hint () } }
};
}
