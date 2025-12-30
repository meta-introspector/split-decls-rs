// Generated macro for other_endpoint (function)
macro_rules! Depcrate_algo_maximum_flow_ford_fulkersonother_endpoint {
() => {
// Module: crate::algo::maximum_flow::ford_fulkerson
// Provides: {"other_endpoint"}
// Dependencies: {}
# [doc = " Gets the other endpoint of graph edge, if any, otherwise panics."] fn other_endpoint < G > (network : G , edge : G :: EdgeRef , vertex : G :: NodeId) -> G :: NodeId where G : NodeIndexable + IntoEdges , { if vertex == edge . source () { edge . target () } else if vertex == edge . target () { edge . source () } else { let end_point = NodeIndexable :: to_index (& network , vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
};
}
