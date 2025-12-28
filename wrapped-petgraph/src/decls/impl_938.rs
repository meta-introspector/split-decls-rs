macro_rules! deps {
    () => {
        EdgeReferences!();
        EdgeType!();
        NodeTrait!();
        GraphMap!();
        AllEdges!();
        EdgeRef!();
    };
}

macro_rules! impl_938 {
    () => {
        deps!();
        impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdgeReferences for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type EdgeRef = (N , N , & 'a E) ; type EdgeReferences = AllEdges < 'a , N , E , Ty > ; fn edge_references (self) -> Self :: EdgeReferences { self . all_edges () } }
    };
}

impl_938!()