macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        NodeReferences!();
        Csr!();
        NodeRef!();
        NodeIndex!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > IntoNodeReferences for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . node_weights . iter () . enumerate () , ty : PhantomData , } } }
    };
}

impl_549!();