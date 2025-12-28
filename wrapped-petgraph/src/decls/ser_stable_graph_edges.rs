macro_rules! deps {
    () => {
        Edge!();
        IndexType!();
    };
}

macro_rules! ser_stable_graph_edges {
    () => {
        deps!();
        fn ser_stable_graph_edges < S , E , Ix > (edges : & & [Edge < Option < E > , Ix >] , serializer : S ,) -> Result < S :: Ok , S :: Error > where S : Serializer , E : Serialize , Ix : Serialize + IndexType , { serializer . collect_seq_exact (edges . iter () . map (| edge | { edge . weight . as_ref () . map (| w | (edge . source () , edge . target () , w)) })) }
    };
}

ser_stable_graph_edges!()