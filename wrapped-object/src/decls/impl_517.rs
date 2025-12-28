macro_rules! deps {
    () => {
        ExportData!();
        Result!();
        Bytes!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < 'data > ExportData < 'data > { pub (super) fn parse (mut data : Bytes < 'data >) -> Result < (u8 , Self) > { let flags = data . read_uleb128 () . read_error ("Invalid exports trie flags") ? ; let flags : u8 = flags . try_into () . map_err (| _ | ()) . read_error ("Exports trie flags too large") ? ; if flags & EXPORT_SYMBOL_FLAGS_REEXPORT != 0 { let dylib_ordinal = data . read_uleb128 () . read_error ("Invalid exports trie dylib ordinal") ? ; let import_name = data . read_string () . read_error ("Invalid exports trie import name") ? ; return Ok ((flags , ExportData :: Reexport { dylib_ordinal , import_name , } ,)) ; } if flags & EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER != 0 { let stub_address = data . read_uleb128 () . read_error ("Invalid exports trie stub address") ? ; let resolver_address = data . read_uleb128 () . read_error ("Invalid exports trie resolver address") ? ; return Ok ((flags , ExportData :: StubAndResolver { stub_address , resolver_address , } ,)) ; } let address = data . read_uleb128 () . read_error ("Invalid exports trie address") ? ; Ok ((flags , ExportData :: Regular { address })) } }
    };
}

impl_517!();