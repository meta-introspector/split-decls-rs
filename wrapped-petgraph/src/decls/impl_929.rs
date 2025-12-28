macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
        NodeIdentifiers!();
    };
}

macro_rules! impl_929 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , S > visit :: IntoNodeIdentifiers for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type NodeIdentifiers = NodeIdentifiers < 'a , N , E , Ty > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers { iter : self . nodes . iter () , ty : self . ty , edge_ty : PhantomData , } } }
    };
}

impl_929!();