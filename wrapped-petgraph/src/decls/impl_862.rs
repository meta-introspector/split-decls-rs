macro_rules! deps {
    () => {
        EdgeReference!();
        EdgeType!();
        Create!();
        EdgeReferences!();
        IndexType!();
        StableGraph!();
        EdgeRef!();
    };
}

macro_rules! impl_862 {
    () => {
        deps!();
        impl < 'a , N : 'a , E : 'a , Ty , Ix > visit :: IntoEdgeReferences for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; # [doc = " Create an iterator over all edges in the graph, in indexed order."] # [doc = ""] # [doc = " Iterator element type is `EdgeReference<E, Ix>`."] fn edge_references (self) -> Self :: EdgeReferences { EdgeReferences { iter : self . g . edges . iter () . enumerate () , } } }
    };
}

impl_862!();