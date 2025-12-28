macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        MatrixGraph!();
        Nullable!();
        NodeIndex!();
    };
}

macro_rules! impl_1030 {
    () => {
        deps!();
        impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > GraphBase for MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeId = NodeIndex < Ix > ; type EdgeId = (NodeIndex < Ix > , NodeIndex < Ix >) ; }
    };
}

impl_1030!()