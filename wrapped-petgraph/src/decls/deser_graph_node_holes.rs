macro_rules! deps {
    () => {
        IndexType!();
        NodeIndex!();
        MappedSequenceVisitor!();
        Graph!();
    };
}

macro_rules! deser_graph_node_holes {
    () => {
        deps!();
        fn deser_graph_node_holes < 'de , D , Ix > (deserializer : D) -> Result < Vec < NodeIndex < Ix > > , D :: Error > where D : Deserializer < 'de > , Ix : IndexType + Deserialize < 'de > , { deserializer . deserialize_seq (MappedSequenceVisitor :: < NodeIndex < Ix > , NodeIndex < Ix > , _ > :: new (| _ | { Err ("Graph can not have holes in the node set, found non-empty node_holes") }) ,) }
    };
}

deser_graph_node_holes!();