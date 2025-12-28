macro_rules! deps {
    () => {
        IndexType!();
        NodeIdentifiers!();
        Nullable!();
        EdgeType!();
        MatrixGraph!();
    };
}

macro_rules! impl_1033 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNodeIdentifiers for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeIdentifiers = NodeIdentifiers < 'a , Ix , S > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers :: new (self . nodes . iter_ids ()) } }
    };
}

impl_1033!();