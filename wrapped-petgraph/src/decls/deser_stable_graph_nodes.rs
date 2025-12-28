macro_rules! deps {
    () => {
        EdgeIndex!();
        Node!();
        IndexType!();
        MappedSequenceVisitor!();
    };
}

macro_rules! deser_stable_graph_nodes {
    () => {
        deps!();
        fn deser_stable_graph_nodes < 'de , D , N , Ix > (deserializer : D ,) -> Result < Vec < Node < Option < N > , Ix > > , D :: Error > where D : Deserializer < 'de > , N : Deserialize < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: new (| n | { Ok (Node { weight : Some (n) , next : [EdgeIndex :: end () ; 2] , }) })) }
    };
}

deser_stable_graph_nodes!();