macro_rules! deps {
    () => {
        NodeIdentifiers!();
        IndexType!();
        EdgeType!();
        StableGraph!();
        NodeIndices!();
    };
}

macro_rules! impl_852 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNodeIdentifiers for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIndices < 'a , N , Ix > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { StableGraph :: node_indices (self) } }
    };
}

impl_852!()