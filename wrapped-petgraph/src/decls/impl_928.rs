macro_rules! deps {
    () => {
        EdgeType!();
        NodeReferences!();
        NodeTrait!();
        NodeRef!();
        GraphMap!();
    };
}

macro_rules! impl_928 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , S > visit :: IntoNodeReferences for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type NodeRef = (N , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , E , Ty > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . nodes . iter () , ty : self . ty , edge_ty : PhantomData , } } }
    };
}

impl_928!()