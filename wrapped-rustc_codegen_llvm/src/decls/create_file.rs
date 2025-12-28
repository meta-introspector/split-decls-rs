macro_rules! deps {
    () => {
        ChecksumKind!();
    };
}

macro_rules! create_file {
    () => {
        deps!();
        fn create_file < 'll > (builder : & DIBuilder < 'll > , file_name : & str , directory : & str , hash_value : & str , hash_kind : llvm :: ChecksumKind , source : Option < & Arc < String > > ,) -> & 'll DIFile { unsafe { llvm :: LLVMRustDIBuilderCreateFile (builder , file_name . as_c_char_ptr () , file_name . len () , directory . as_c_char_ptr () , directory . len () , hash_kind , hash_value . as_c_char_ptr () , hash_value . len () , source . map_or (ptr :: null () , | x | x . as_c_char_ptr ()) , source . map_or (0 , | x | x . len ()) ,) } }
    };
}

create_file!()