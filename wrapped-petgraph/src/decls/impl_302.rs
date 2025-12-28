macro_rules! deps {
    () => {
        NodeIdentifiers!();
        List!();
        IndexType!();
        NodeIndices!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < E , Ix : IndexType > visit :: IntoNodeIdentifiers for & List < E , Ix > { type NodeIdentifiers = NodeIndices < Ix > ; fn node_identifiers (self) -> NodeIndices < Ix > { self . node_indices () } }
    };
}

impl_302!();