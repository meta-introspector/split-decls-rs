macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
        NodeIndex!();
        Csr!();
        EdgeType!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > GraphBase for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex ; }
    };
}

impl_534!()