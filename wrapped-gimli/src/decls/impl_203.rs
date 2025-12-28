macro_rules! deps {
    () => {
        UnwindSection!();
        Format!();
        CommonInformationEntry!();
        Reader!();
        Register!();
        Error!();
        BaseAddresses!();
        Section!();
        Augmentation!();
        CieOrFde!();
        Result!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < R : Reader > CommonInformationEntry < R > { fn parse < Section : UnwindSection < R > > (bases : & BaseAddresses , section : & Section , input : & mut R ,) -> Result < CommonInformationEntry < R > > { match parse_cfi_entry (bases , section , input) ? { Some (CieOrFde :: Cie (cie)) => Ok (cie) , Some (CieOrFde :: Fde (_)) => Err (Error :: NotCieId) , None => Err (Error :: NoEntryAtGivenOffset) , } } fn parse_rest < Section : UnwindSection < R > > (offset : R :: Offset , length : R :: Offset , format : Format , bases : & BaseAddresses , section : & Section , mut rest : R ,) -> Result < CommonInformationEntry < R > > { let version = rest . read_u8 () ? ; match version { 1 | 3 | 4 => () , _ => return Err (Error :: UnknownVersion (u64 :: from (version))) , } let mut augmentation_string = rest . read_null_terminated_slice () ? ; let address_size = if Section :: has_address_and_segment_sizes (version) { let address_size = rest . read_address_size () ? ; let segment_size = rest . read_u8 () ? ; if segment_size != 0 { return Err (Error :: UnsupportedSegmentSize) ; } address_size } else { section . address_size () } ; let code_alignment_factor = rest . read_uleb128 () ? ; let data_alignment_factor = rest . read_sleb128 () ? ; let return_address_register = if version == 1 { Register (rest . read_u8 () ? . into ()) } else { rest . read_uleb128 () . and_then (Register :: from_u64) ? } ; let augmentation = if augmentation_string . is_empty () { None } else { Some (Augmentation :: parse (& mut augmentation_string , bases , address_size , section , & mut rest ,) ?) } ; let entry = CommonInformationEntry { offset , length , format , version , augmentation , address_size , code_alignment_factor , data_alignment_factor , return_address_register , initial_instructions : rest , } ; Ok (entry) } }
    };
}

impl_203!()