// Generated macro for impl_384 (impl)
macro_rules! Depcrate_algo_articulation_pointsimpl_384 {
() => {
// Module: crate::algo::articulation_points
// Provides: {"impl_384"}
// Dependencies: {}
impl ArticulationPointTracker { fn new (graph_size : usize) -> Self { Self { visited : FixedBitSet :: with_capacity (graph_size) , low : vec ! [usize :: MAX ; graph_size] , disc : vec ! [usize :: MAX ; graph_size] , parent : vec ! [usize :: MAX ; graph_size] , articulation_points : HashSet :: with_capacity (graph_size) , time : 0 , } } }
};
}
