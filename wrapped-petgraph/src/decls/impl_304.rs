macro_rules! deps {
    () => {
        NodeIndices!();
        NodeRef!();
        NodeReferences!();
        NodeIndex!();
        IndexType!();
        List!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < Ix : IndexType , E > visit :: IntoNodeReferences for & List < E , Ix > { type NodeRef = NodeIndex < Ix > ; type NodeReferences = NodeIndices < Ix > ; fn node_references (self) -> Self :: NodeReferences { self . node_indices () } }
    };
}

impl_304!();