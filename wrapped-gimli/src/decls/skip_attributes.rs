macro_rules! deps {
    () => {
        Encoding!();
        Reader!();
        Result!();
        AttributeSpecification!();
        Error!();
    };
}

macro_rules! skip_attributes {
    () => {
        deps!();
        pub (crate) fn skip_attributes < R : Reader > (input : & mut R , encoding : Encoding , specs : & [AttributeSpecification] ,) -> Result < () > { let mut skip_bytes = R :: Offset :: from_u8 (0) ; for spec in specs { let mut form = spec . form () ; loop { if let Some (len) = get_attribute_size (form , encoding) { skip_bytes += R :: Offset :: from_u8 (len) ; break ; } if skip_bytes != R :: Offset :: from_u8 (0) { input . skip (skip_bytes) ? ; skip_bytes = R :: Offset :: from_u8 (0) ; } match form { constants :: DW_FORM_indirect => { let dynamic_form = input . read_uleb128_u16 () ? ; form = constants :: DwForm (dynamic_form) ; continue ; } constants :: DW_FORM_block1 => { skip_bytes = input . read_u8 () . map (R :: Offset :: from_u8) ? ; } constants :: DW_FORM_block2 => { skip_bytes = input . read_u16 () . map (R :: Offset :: from_u16) ? ; } constants :: DW_FORM_block4 => { skip_bytes = input . read_u32 () . map (R :: Offset :: from_u32) ? ; } constants :: DW_FORM_block | constants :: DW_FORM_exprloc => { skip_bytes = input . read_uleb128 () . and_then (R :: Offset :: from_u64) ? ; } constants :: DW_FORM_string => { let _ = input . read_null_terminated_slice () ? ; } constants :: DW_FORM_udata | constants :: DW_FORM_sdata | constants :: DW_FORM_ref_udata | constants :: DW_FORM_strx | constants :: DW_FORM_GNU_str_index | constants :: DW_FORM_addrx | constants :: DW_FORM_GNU_addr_index | constants :: DW_FORM_loclistx | constants :: DW_FORM_rnglistx => { input . skip_leb128 () ? ; } _ => { return Err (Error :: UnknownForm (form)) ; } } ; break ; } } if skip_bytes != R :: Offset :: from_u8 (0) { input . skip (skip_bytes) ? ; } Ok (()) }
    };
}

skip_attributes!()