macro_rules! deps {
    () => {
        NodeIdentifiers!();
        IndexType!();
        EdgeType!();
        Csr!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > IntoNodeIdentifiers for & Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIdentifiers < Ix > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers { r : 0 .. self . node_count () , ty : PhantomData , } } }
    };
}

impl_545!();