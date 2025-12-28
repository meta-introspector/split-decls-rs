macro_rules! deps {
    () => {
        IndexType!();
        Node!();
    };
}

macro_rules! ser_graph_nodes {
    () => {
        deps!();
        fn ser_graph_nodes < S , N , Ix > (nodes : & & [Node < N , Ix >] , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , N : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (nodes . iter () . map (| node | & node . weight)) }
    };
}

ser_graph_nodes!()