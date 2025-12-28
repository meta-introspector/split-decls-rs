macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! compute_shortest_path_length {
    () => {
        deps!();
        fn compute_shortest_path_length < G > (graph : G , source : G :: NodeId , target : G :: NodeId) -> G :: EdgeWeight where G : Visitable + IntoEdges , G :: NodeId : Eq + Hash , G :: EdgeWeight : Measure + Copy , { let output = dijkstra (graph , source , Some (target) , | e | * e . weight ()) ; output [& target] }
    };
}

compute_shortest_path_length!()