macro_rules! deps {
    () => {
        NodeRef!();
        NodeReferences!();
        FilterNode!();
        NodeFilteredNodes!();
        NodeFiltered!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNodeReferences for & 'a NodeFiltered < G , F > where G : IntoNodeReferences , F : FilterNode < G :: NodeId > , { type NodeRef = G :: NodeRef ; type NodeReferences = NodeFilteredNodes < 'a , G :: NodeReferences , F > ; fn node_references (self) -> Self :: NodeReferences { NodeFilteredNodes { include_source : true , iter : self . 0 . node_references () , f : & self . 1 , } } }
    };
}

impl_125!();