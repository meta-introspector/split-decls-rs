macro_rules! deps {
    () => {
        Edge!();
        IndexType!();
    };
}

macro_rules! ser_graph_edges {
    () => {
        deps!();
        fn ser_graph_edges < S , E , Ix > (edges : & & [Edge < E , Ix >] , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , E : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (edges . iter () . map (| edge | Some ((edge . source () , edge . target () , & edge . weight))) ,) }
    };
}

ser_graph_edges!();