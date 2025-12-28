macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        Undirected!();
        FromGraph6!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl < Ix : IndexType > FromGraph6 for Graph < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : Graph < () , () , Undirected , Ix > = Graph :: with_capacity (order , edges . len ()) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges) ; graph } }
    };
}

impl_605!()