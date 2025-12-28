macro_rules! deps {
    () => {
        DebugTypes!();
        UnitHeader!();
        Encoding!();
        UnitType!();
        UnitSectionOffset!();
        Reader!();
        Result!();
        Error!();
        SectionId!();
        ReaderOffset!();
    };
}

macro_rules! parse_unit_header {
    () => {
        deps!();
        # [doc = " Parse a unit header."] fn parse_unit_header < R , Offset > (input : & mut R , section : SectionId , unit_offset : UnitSectionOffset < Offset > ,) -> Result < UnitHeader < R > > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { let (unit_length , format) = input . read_initial_length () ? ; let mut rest = input . split (unit_length) ? ; let version = rest . read_u16 () ? ; let abbrev_offset ; let address_size ; let unit_type ; if 2 <= version && version <= 4 { abbrev_offset = parse_debug_abbrev_offset (& mut rest , format) ? ; address_size = rest . read_address_size () ? ; unit_type = match section { SectionId :: DebugTypes => constants :: DW_UT_type , _ => constants :: DW_UT_compile , } ; } else if version == 5 { unit_type = parse_unit_type (& mut rest) ? ; address_size = rest . read_address_size () ? ; abbrev_offset = parse_debug_abbrev_offset (& mut rest , format) ? ; } else { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let encoding = Encoding { format , version , address_size , } ; let unit_type = match unit_type { constants :: DW_UT_compile => UnitType :: Compilation , constants :: DW_UT_type => { let type_signature = parse_type_signature (& mut rest) ? ; let type_offset = parse_type_offset (& mut rest , format) ? ; UnitType :: Type { type_signature , type_offset , } } constants :: DW_UT_partial => UnitType :: Partial , constants :: DW_UT_skeleton => { let dwo_id = parse_dwo_id (& mut rest) ? ; UnitType :: Skeleton (dwo_id) } constants :: DW_UT_split_compile => { let dwo_id = parse_dwo_id (& mut rest) ? ; UnitType :: SplitCompilation (dwo_id) } constants :: DW_UT_split_type => { let type_signature = parse_type_signature (& mut rest) ? ; let type_offset = parse_type_offset (& mut rest , format) ? ; UnitType :: SplitType { type_signature , type_offset , } } _ => return Err (Error :: UnsupportedUnitType) , } ; Ok (UnitHeader :: new (encoding , unit_length , unit_type , abbrev_offset , section , unit_offset , rest ,)) }
    };
}

parse_unit_header!()