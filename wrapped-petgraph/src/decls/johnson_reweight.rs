macro_rules! deps {
    () => {
        NegativeCycle!();
        EdgeRef!();
        BoundedMeasure!();
    };
}

macro_rules! johnson_reweight {
    () => {
        deps!();
        # [doc = " Add a virtual node to the graph with oriented edges with zero weight"] # [doc = " to all other vertices, and then run SPFA from it."] # [doc = " The found distances will be used to change the edge weights in Dijkstra's"] # [doc = " algorithm to make them non-negative."] fn johnson_reweight < G , F , K > (graph : G , mut edge_cost : F) -> Result < Vec < K > , NegativeCycle > where G : IntoEdges + IntoNodeIdentifiers + NodeIndexable + Visitable , G :: NodeId : Eq + Hash , F : FnMut (G :: EdgeRef) -> K , K : BoundedMeasure + Copy + Sub < K , Output = K > , { let node_bound = graph . node_bound () ; let reweight = vec ! [K :: default () ; node_bound] ; let mut queue : VecDeque < G :: NodeId > = VecDeque :: with_capacity (node_bound) ; queue . extend (graph . node_identifiers ()) ; let in_queue = vec ! [true ; node_bound] ; spfa_loop (graph , reweight , None , queue , in_queue , & mut edge_cost) . map (| (dists , _) | dists) }
    };
}

johnson_reweight!()