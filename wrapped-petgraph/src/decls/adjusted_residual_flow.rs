macro_rules! deps {
    () => {
        PositiveMeasure!();
        EdgeRef!();
    };
}

macro_rules! adjusted_residual_flow {
    () => {
        deps!();
        # [doc = " Returns the adjusted residual flow for given edge and flow increase."] fn adjusted_residual_flow < G > (network : G , edge : G :: EdgeRef , target_vertex : G :: NodeId , flow : G :: EdgeWeight , flow_increase : G :: EdgeWeight ,) -> G :: EdgeWeight where G : NodeIndexable + IntoEdges , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { if target_vertex == edge . source () { flow - flow_increase } else if target_vertex == edge . target () { flow + flow_increase } else { let end_point = NodeIndexable :: to_index (& network , target_vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
    };
}

adjusted_residual_flow!()