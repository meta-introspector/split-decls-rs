macro_rules! deps {
    () => {
        MatrixGraph!();
        EdgeType!();
        NodeIndex!();
        Nullable!();
        IndexType!();
    };
}

macro_rules! impl_1028 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > GetAdjacencyMatrix for MatrixGraph < N , E , S , Ty , Null , Ix > { type AdjMatrix = () ; fn adjacency_matrix (& self) -> Self :: AdjMatrix { } fn is_adjacent (& self , _ : & Self :: AdjMatrix , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { MatrixGraph :: has_edge (self , a , b) } }
    };
}

impl_1028!();