// Generated macro for impl_500 (impl)
macro_rules! Depcrate_flowgraphimpl_500 {
() => {
// Module: crate::flowgraph
// Provides: {"impl_500"}
// Dependencies: {}
impl < 'a > Iterator for PredIter < 'a > { type Item = BlockPredecessor ; fn next (& mut self) -> Option < BlockPredecessor > { self . 0 . next () . map (| (i , e) | BlockPredecessor :: new (e , i)) } }
};
}
