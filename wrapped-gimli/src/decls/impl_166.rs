macro_rules! deps {
    () => {
        EhFrameHdr!();
        Error!();
        Result!();
        Reader!();
        BaseAddresses!();
        PointerEncodingParameters!();
        ParsedEhFrameHdr!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < R : Reader > EhFrameHdr < R > { # [doc = " Parses this `EhFrameHdr` to a `ParsedEhFrameHdr`."] pub fn parse (& self , bases : & BaseAddresses , address_size : u8) -> Result < ParsedEhFrameHdr < R > > { let mut reader = self . 0 . clone () ; let version = reader . read_u8 () ? ; if version != 1 { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let eh_frame_ptr_enc = parse_pointer_encoding (& mut reader) ? ; let fde_count_enc = parse_pointer_encoding (& mut reader) ? ; let table_enc = parse_pointer_encoding (& mut reader) ? ; let parameters = PointerEncodingParameters { bases : & bases . eh_frame_hdr , func_base : None , address_size , section : & self . 0 , } ; if eh_frame_ptr_enc == constants :: DW_EH_PE_omit { return Err (Error :: CannotParseOmitPointerEncoding) ; } let eh_frame_ptr = parse_encoded_pointer (eh_frame_ptr_enc , & parameters , & mut reader) ? ; let fde_count ; if fde_count_enc == constants :: DW_EH_PE_omit || table_enc == constants :: DW_EH_PE_omit { fde_count = 0 } else { if fde_count_enc != fde_count_enc . format () { return Err (Error :: UnsupportedPointerEncoding) ; } fde_count = parse_encoded_value (fde_count_enc , & parameters , & mut reader) ? ; } Ok (ParsedEhFrameHdr { address_size , section : self . 0 . clone () , eh_frame_ptr , fde_count , table_enc , table : reader , }) } }
    };
}

impl_166!();