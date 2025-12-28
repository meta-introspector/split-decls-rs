macro_rules! deps {
    () => {
        IndexType!();
        Edge!();
        EdgeIndex!();
        NodeIndex!();
        MappedSequenceVisitor!();
    };
}

macro_rules! deser_stable_graph_edges {
    () => {
        deps!();
        fn deser_stable_graph_edges < 'de , D , N , Ix > (deserializer : D ,) -> Result < Vec < Edge < Option < N > , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: < Option < (NodeIndex < Ix > , NodeIndex < Ix > , N) > , _ , _ , > :: new (| x | { if let Some ((i , j , w)) = x { Ok (Edge { weight : Some (w) , node : [i , j] , next : [EdgeIndex :: end () ; 2] , }) } else { Ok (Edge { weight : None , node : [NodeIndex :: end () ; 2] , next : [EdgeIndex :: end () ; 2] , }) } })) }
    };
}

deser_stable_graph_edges!()