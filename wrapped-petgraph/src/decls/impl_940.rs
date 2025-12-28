macro_rules! deps {
    () => {
        EdgeType!();
        GraphMap!();
    };
}

macro_rules! impl_940 {
    () => {
        deps!();
        # [doc = " The `GraphMap` keeps an adjacency matrix internally."] impl < N , E , Ty , S > visit :: GetAdjacencyMatrix for GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type AdjMatrix = () ; # [inline] fn adjacency_matrix (& self) { } # [inline] fn is_adjacent (& self , _ : & () , a : N , b : N) -> bool { self . contains_edge (a , b) } }
    };
}

impl_940!()