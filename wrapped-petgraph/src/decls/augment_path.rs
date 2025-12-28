macro_rules! deps {
    () => {
        Edge!();
        Label!();
    };
}

macro_rules! augment_path {
    () => {
        deps!();
        fn augment_path < G > (graph : & G , outer : G :: NodeId , other : G :: NodeId , mate : & mut [Option < G :: NodeId >] , label : & [Label < G >] ,) where G : NodeIndexable , { let outer_idx = graph . to_index (outer) ; let temp = mate [outer_idx] ; let temp_idx = temp . map_or (graph . dummy_idx () , | id | graph . to_index (id)) ; mate [outer_idx] = Some (other) ; if mate [temp_idx] != Some (outer) { } else if let Label :: Vertex (vertex) = label [outer_idx] { mate [temp_idx] = Some (vertex) ; if let Some (temp) = temp { augment_path (graph , vertex , temp , mate , label) ; } } else if let Label :: Edge (_ , [source , target]) = label [outer_idx] { augment_path (graph , source , target , mate , label) ; augment_path (graph , target , source , mate , label) ; } else { panic ! ("Unexpected label when augmenting path") ; } }
    };
}

augment_path!()