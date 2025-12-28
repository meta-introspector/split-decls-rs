macro_rules! get_metadata_xcoff {
    () => {
        pub (super) fn get_metadata_xcoff < 'a > (path : & Path , data : & 'a [u8]) -> Result < & 'a [u8] , String > { let Ok (file) = object :: File :: parse (data) else { return Ok (data) ; } ; let info_data = search_for_section (path , data , ".info") ? ; if let Some (metadata_symbol) = file . symbols () . find (| sym | sym . name () == Ok (AIX_METADATA_SYMBOL_NAME)) { let offset = metadata_symbol . address () as usize ; if offset < 4 { return Err (format ! ("Invalid metadata symbol offset: {offset}")) ; } let len = u32 :: from_be_bytes (info_data [(offset - 4) .. offset] . try_into () . unwrap ()) as usize ; if offset + len > (info_data . len () as usize) { return Err (format ! ("Metadata at offset {offset} with size {len} is beyond .info section")) ; } Ok (& info_data [offset .. (offset + len)]) } else { Err (format ! ("Unable to find symbol {AIX_METADATA_SYMBOL_NAME}")) } }
    };
}

get_metadata_xcoff!()