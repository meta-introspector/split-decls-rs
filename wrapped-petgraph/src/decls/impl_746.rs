macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        NodeRef!();
        NodeIndex!();
        NodeReferences!();
        EdgeType!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoNodeReferences for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . nodes . iter () . enumerate () , } } }
    };
}

impl_746!();