macro_rules! deps {
    () => {
        NodeReferences!();
        IndexType!();
        MatrixGraph!();
        NodeIndex!();
        EdgeType!();
        NodeRef!();
        Nullable!();
    };
}

macro_rules! impl_1036 {
    () => {
        deps!();
        impl < 'a , N , E , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType , S : BuildHasher + 'a > IntoNodeReferences for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix , S > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences :: new (& self . nodes) } }
    };
}

impl_1036!();