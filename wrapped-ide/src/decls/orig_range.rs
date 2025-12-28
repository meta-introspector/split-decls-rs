macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! orig_range {
    () => {
        deps!();
        fn orig_range (db : & RootDatabase , hir_file : HirFileId , value : & SyntaxNode ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { UpmappingResult { call_site : (InFile :: new (hir_file , value) . original_file_range_rooted (db) . into_file_id (db) , None ,) , def_site : None , } }
    };
}

orig_range!();