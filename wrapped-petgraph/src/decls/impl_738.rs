macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        NodeIdentifiers!();
        NodeIndices!();
        Graph!();
    };
}

macro_rules! impl_738 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNodeIdentifiers for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIndices < Ix > ; fn node_identifiers (self) -> NodeIndices < Ix > { Graph :: node_indices (self) } }
    };
}

impl_738!();