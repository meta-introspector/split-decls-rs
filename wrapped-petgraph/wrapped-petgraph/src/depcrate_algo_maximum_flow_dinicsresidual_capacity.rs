// Generated macro for residual_capacity (function)
macro_rules! Depcrate_algo_maximum_flow_dinicsresidual_capacity {
() => {
// Module: crate::algo::maximum_flow::dinics
// Provides: {"residual_capacity"}
// Dependencies: {}
# [doc = " Returns the residual capacity of given edge."] fn residual_capacity < G > (network : G , edge : G :: EdgeRef , target_vertex : G :: NodeId , flow : G :: EdgeWeight ,) -> G :: EdgeWeight where G : NodeIndexable + IntoEdges , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { if target_vertex == edge . source () { flow } else if target_vertex == edge . target () { * edge . weight () - flow } else { let end_point = NodeIndexable :: to_index (& network , target_vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
};
}
