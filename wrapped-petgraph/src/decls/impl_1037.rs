macro_rules! deps {
    () => {
        MatrixGraph!();
        EdgeType!();
        IndexType!();
        EdgeRef!();
        Nullable!();
        EdgeReferences!();
        NodeIndex!();
    };
}

macro_rules! impl_1037 {
    () => {
        deps!();
        impl < 'a , N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdgeReferences for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type EdgeRef = (NodeIndex < Ix > , NodeIndex < Ix > , & 'a E) ; type EdgeReferences = EdgeReferences < 'a , Ty , Null , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { EdgeReferences :: new (& self . node_adjacencies , self . node_capacity) } }
    };
}

impl_1037!()