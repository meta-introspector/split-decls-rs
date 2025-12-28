macro_rules! deps {
    () => {
        EdgeRef!();
    };
}

macro_rules! other_endpoint {
    () => {
        deps!();
        # [doc = " Gets the other endpoint of graph edge, if any, otherwise panics."] fn other_endpoint < G > (network : G , edge : G :: EdgeRef , vertex : G :: NodeId) -> G :: NodeId where G : NodeIndexable + IntoEdges , { if vertex == edge . source () { edge . target () } else if vertex == edge . target () { edge . source () } else { let end_point = NodeIndexable :: to_index (& network , vertex) ; panic ! ("Illegal endpoint {}" , end_point) ; } }
    };
}

other_endpoint!();