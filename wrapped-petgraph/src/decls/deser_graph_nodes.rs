macro_rules! deps {
    () => {
        Node!();
        IndexType!();
        EdgeIndex!();
        MappedSequenceVisitor!();
    };
}

macro_rules! deser_graph_nodes {
    () => {
        deps!();
        fn deser_graph_nodes < 'de , D , N , Ix > (deserializer : D) -> Result < Vec < Node < N , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: new (| n | { Ok (Node { weight : n , next : [EdgeIndex :: end () ; 2] , }) })) }
    };
}

deser_graph_nodes!();