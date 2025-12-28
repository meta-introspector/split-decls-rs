macro_rules! deps {
    () => {
        DefaultMetadataLoader!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl MetadataLoader for DefaultMetadataLoader { fn get_rlib_metadata (& self , target : & Target , path : & Path) -> Result < OwnedSlice , String > { debug ! ("getting rlib metadata for {}" , path . display ()) ; load_metadata_with (path , | data | { let archive = object :: read :: archive :: ArchiveFile :: parse (& * data) . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; for entry_result in archive . members () { let entry = entry_result . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; if entry . name () == METADATA_FILENAME . as_bytes () { let data = entry . data (data) . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; if target . is_like_aix { return get_metadata_xcoff (path , data) ; } else { return search_for_section (path , data , ".rmeta") ; } } } Err (format ! ("metadata not found in rlib '{}'" , path . display ())) }) } fn get_dylib_metadata (& self , target : & Target , path : & Path) -> Result < OwnedSlice , String > { debug ! ("getting dylib metadata for {}" , path . display ()) ; if target . is_like_aix { load_metadata_with (path , | data | { let archive = object :: read :: archive :: ArchiveFile :: parse (& * data) . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; match archive . members () . exactly_one () { Ok (lib) => { let lib = lib . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; let data = lib . data (data) . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; get_metadata_xcoff (path , data) } Err (e) => Err (format ! ("failed to parse aix dylib '{}': {}" , path . display () , e)) , } }) } else { load_metadata_with (path , | data | search_for_section (path , data , ".rustc")) } } }
    };
}

impl_155!();