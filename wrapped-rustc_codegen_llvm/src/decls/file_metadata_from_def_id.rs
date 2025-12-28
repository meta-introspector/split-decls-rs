macro_rules! deps {
    () => {
        CodegenCx!();
        DefinitionLocation!();
    };
}

macro_rules! file_metadata_from_def_id {
    () => {
        deps!();
        pub (crate) fn file_metadata_from_def_id < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : Option < DefId > ,) -> DefinitionLocation < 'll > { if let Some (def_id) = def_id && let span = hygiene :: walk_chain_collapsed (cx . tcx . def_span (def_id) , DUMMY_SP) && ! span . is_dummy () { let loc = cx . lookup_debug_loc (span . lo ()) ; (file_metadata (cx , & loc . file) , loc . line) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } }
    };
}

file_metadata_from_def_id!()