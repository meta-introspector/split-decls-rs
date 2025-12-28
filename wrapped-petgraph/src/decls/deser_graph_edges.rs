macro_rules! deps {
    () => {
        NodeIndex!();
        Edge!();
        EdgeIndex!();
        MappedSequenceVisitor!();
        IndexType!();
        Graph!();
    };
}

macro_rules! deser_graph_edges {
    () => {
        deps!();
        fn deser_graph_edges < 'de , D , N , Ix > (deserializer : D) -> Result < Vec < Edge < N , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: < Option < (NodeIndex < Ix > , NodeIndex < Ix > , N) > , _ , _ , > :: new (| x | { if let Some ((i , j , w)) = x { Ok (Edge { weight : w , node : [i , j] , next : [EdgeIndex :: end () ; 2] , }) } else { Err ("Graph can not have holes in the edge set, found None, expected edge") } })) }
    };
}

deser_graph_edges!();