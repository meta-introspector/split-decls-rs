// Generated macro for adjust_residual_flow (function)
macro_rules! Depcrate_algo_maximum_flow_ford_fulkersonadjust_residual_flow {
() => {
// Module: crate::algo::maximum_flow::ford_fulkerson
// Provides: {"adjust_residual_flow"}
// Dependencies: {}
fn adjust_residual_flow < G > (network : G , edge : G :: EdgeRef , vertex : G :: NodeId , flow : G :: EdgeWeight , delta : G :: EdgeWeight ,) -> G :: EdgeWeight where G : NodeIndexable + IntoEdges , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { if vertex == edge . source () { flow - delta } else if vertex == edge . target () { flow + delta } else { let end_point = NodeIndexable :: to_index (& network , vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
};
}
