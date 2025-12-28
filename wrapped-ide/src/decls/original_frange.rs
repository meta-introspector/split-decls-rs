macro_rules! original_frange {
    () => {
        fn original_frange (db : & dyn db :: ExpandDatabase , file_id : HirFileId , text_range : Option < TextRange > ,) -> Option < FileRange > { InFile :: new (file_id , text_range ?) . original_node_file_range_opt (db) . map (| (frange , _) | frange) }
    };
}

original_frange!()