macro_rules! deps {
    () => {
        IndexType!();
        Undirected!();
        GraphMap!();
        FromGraph6!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        # [cfg (feature = "graphmap")] impl < Ix : IndexType , S : BuildHasher + Default > FromGraph6 for GraphMap < Ix , () , Undirected , S > { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : GraphMap < Ix , () , Undirected , S > = GraphMap :: with_capacity (order , edges . len ()) ; for i in 0 .. order { graph . add_node (Ix :: new (i)) ; } for (a , b) in edges { graph . add_edge (a , b , ()) ; } graph } }
    };
}

impl_607!()