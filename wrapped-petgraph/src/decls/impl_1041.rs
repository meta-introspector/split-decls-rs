macro_rules! deps {
    () => {
        Nullable!();
        MatrixGraph!();
        Build!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_1041 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Build for MatrixGraph < N , E , S , Ty , Null , Ix > { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId { self . add_node (weight) } fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { if ! self . has_edge (a , b) { MatrixGraph :: update_edge (self , a , b , weight) ; Some ((a , b)) } else { None } } fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId { MatrixGraph :: update_edge (self , a , b , weight) ; (a , b) } }
    };
}

impl_1041!();