macro_rules! deps {
    () => {
        EdgeType!();
        EdgeReference!();
        Graph!();
        EdgeRef!();
        EdgeReferences!();
        IndexType!();
    };
}

macro_rules! impl_744 {
    () => {
        deps!();
        impl < 'a , N : 'a , E : 'a , Ty , Ix > visit :: IntoEdgeReferences for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { (* self) . edge_references () } }
    };
}

impl_744!()