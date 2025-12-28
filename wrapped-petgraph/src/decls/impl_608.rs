macro_rules! deps {
    () => {
        Nullable!();
        FromGraph6!();
        IndexType!();
        MatrixGraph!();
        Undirected!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        # [cfg (feature = "matrix_graph")] impl < Null , Ix , S > FromGraph6 for MatrixGraph < () , () , S , Undirected , Null , Ix > where Null : Nullable < Wrapped = () > , Ix : IndexType , S : BuildHasher + Default , { fn from_graph6_string (graph6_string : String) -> Self { let (order , edges) : (usize , Vec < (Ix , Ix) >) = from_graph6_representation (graph6_string) ; let mut graph : MatrixGraph < () , () , S , Undirected , Null , Ix > = MatrixGraph :: with_capacity (order) ; for _ in 0 .. order { graph . add_node (()) ; } graph . extend_with_edges (edges . iter ()) ; graph } }
    };
}

impl_608!()