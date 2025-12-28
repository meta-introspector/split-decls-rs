macro_rules! deps {
    () => {
        CodegenCx!();
        ChecksumKind!();
        DIB!();
    };
}

macro_rules! unknown_file_metadata {
    () => {
        deps!();
        fn unknown_file_metadata < 'll > (cx : & CodegenCx < 'll , '_ >) -> & 'll DIFile { debug_context (cx) . created_files . borrow_mut () . entry (None) . or_insert_with (| | { create_file (DIB (cx) , "<unknown>" , "" , "" , llvm :: ChecksumKind :: None , None) }) }
    };
}

unknown_file_metadata!();