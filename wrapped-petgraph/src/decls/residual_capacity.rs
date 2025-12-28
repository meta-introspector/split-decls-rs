macro_rules! deps {
    () => {
        EdgeRef!();
        PositiveMeasure!();
    };
}

macro_rules! residual_capacity {
    () => {
        deps!();
        fn residual_capacity < G > (network : G , edge : G :: EdgeRef , vertex : G :: NodeId , flow : G :: EdgeWeight ,) -> G :: EdgeWeight where G : NodeIndexable + IntoEdges , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { if vertex == edge . source () { flow } else if vertex == edge . target () { * edge . weight () - flow } else { let end_point = NodeIndexable :: to_index (& network , vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
    };
}

residual_capacity!()