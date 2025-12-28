macro_rules! deps {
    () => {
        EdgeType!();
        NodeReferences!();
        NodeIndex!();
        StableGraph!();
        IndexType!();
        NodeRef!();
    };
}

macro_rules! impl_854 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoNodeReferences for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . raw_nodes () . iter () . enumerate () , } } }
    };
}

impl_854!()