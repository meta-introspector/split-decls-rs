// Generated macro for EdgeCrossOptimizer (struct)
macro_rules! Depcrate_topo_optimizerEdgeCrossOptimizer {
() => {
// Module: crate::topo::optimizer
// Provides: {"EdgeCrossOptimizer"}
// Dependencies: {}
# [doc = " This optimizations changes the order of nodes within a rank (ordering along"] # [doc = " the x-axis). The transformation tries to reduce the number of edges that"] # [doc = " cross each other."] # [derive (Debug)] pub struct EdgeCrossOptimizer < 'a > { dag : & 'a mut DAG , }
};
}
