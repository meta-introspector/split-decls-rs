macro_rules! deps {
    () => {
        PositiveMeasure!();
        EdgeRef!();
    };
}

macro_rules! adjust_residual_flow {
    () => {
        deps!();
        fn adjust_residual_flow < G > (network : G , edge : G :: EdgeRef , vertex : G :: NodeId , flow : G :: EdgeWeight , delta : G :: EdgeWeight ,) -> G :: EdgeWeight where G : NodeIndexable + IntoEdges , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { if vertex == edge . source () { flow - delta } else if vertex == edge . target () { flow + delta } else { let end_point = NodeIndexable :: to_index (& network , vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
    };
}

adjust_residual_flow!();