macro_rules! deps {
    () => {
        FromGraph6!();
        Undirected!();
        Csr!();
        IndexType!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl < Ix : IndexType > FromGraph6 for Csr < () , () , Undirected , Ix > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : Csr < () , () , Undirected , Ix > = Csr :: new () ; let mut nodes = Vec :: new () ; for _ in 0 .. order { let i = graph . add_node (()) ; nodes . push (i) ; } for (a , b) in edges { graph . add_edge (a , b , ()) ; } graph } }
    };
}

impl_609!()