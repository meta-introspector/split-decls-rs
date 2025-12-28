macro_rules! deps {
    () => {
        FromGraph6!();
        StableGraph!();
        IndexType!();
        Undirected!();
        StableUnGraph!();
    };
}

macro_rules! impl_606 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < Ix : IndexType > FromGraph6 for StableGraph < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : StableGraph < () , () , Undirected , Ix > = StableUnGraph :: with_capacity (order , edges . len ()) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges) ; graph } }
    };
}

impl_606!();