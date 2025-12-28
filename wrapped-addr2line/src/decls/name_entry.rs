macro_rules! deps {
    () => {
        Error!();
        Context!();
        DebugFile!();
        Result!();
        UnitRef!();
    };
}

macro_rules! name_entry {
    () => {
        deps!();
        fn name_entry < R > (file : DebugFile , unit : gimli :: UnitRef < R > , offset : gimli :: UnitOffset < R :: Offset > , ctx : & Context < R > , recursion_limit : usize ,) -> Result < Option < R > , Error > where R : gimli :: Reader , { let mut entries = unit . entries_raw (Some (offset)) ? ; let abbrev = if let Some (abbrev) = entries . read_abbreviation () ? { abbrev } else { return Err (gimli :: Error :: NoEntryAtGivenOffset) ; } ; let mut name = None ; let mut next = None ; for spec in abbrev . attributes () { match entries . read_attribute (* spec) { Ok (ref attr) => match attr . name () { gimli :: DW_AT_linkage_name | gimli :: DW_AT_MIPS_linkage_name => { if let Ok (val) = unit . attr_string (attr . value ()) { return Ok (Some (val)) ; } } gimli :: DW_AT_name => { if let Ok (val) = unit . attr_string (attr . value ()) { name = Some (val) ; } } gimli :: DW_AT_abstract_origin | gimli :: DW_AT_specification => { next = Some (attr . value ()) ; } _ => { } } , Err (e) => return Err (e) , } } if name . is_some () { return Ok (name) ; } if let Some (next) = next { return name_attr (next , file , unit , ctx , recursion_limit - 1) ; } Ok (None) }
    };
}

name_entry!()