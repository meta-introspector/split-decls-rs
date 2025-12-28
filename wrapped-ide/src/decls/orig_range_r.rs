macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! orig_range_r {
    () => {
        deps!();
        fn orig_range_r (db : & RootDatabase , hir_file : HirFileId , value : TextRange ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { UpmappingResult { call_site : (InFile :: new (hir_file , value) . original_node_file_range (db) . 0 . into_file_id (db) , None ,) , def_site : None , } }
    };
}

orig_range_r!()