macro_rules! deps {
    () => {
        Context!();
        Error!();
        DebugFile!();
        Result!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < R : gimli :: Reader > Context < R > { fn find_unit (& self , offset : gimli :: DebugInfoOffset < R :: Offset > , file : DebugFile ,) -> Result < (& gimli :: Unit < R > , gimli :: UnitOffset < R :: Offset >) , Error > { let unit = match file { DebugFile :: Primary => self . units . find_offset (offset) ? , DebugFile :: Supplementary => self . sup_units . find_offset (offset) ? , DebugFile :: Dwo => return Err (gimli :: Error :: NoEntryAtGivenOffset) , } ; let unit_offset = offset . to_unit_offset (& unit . header) . ok_or (gimli :: Error :: NoEntryAtGivenOffset) ? ; Ok ((unit , unit_offset)) } }
    };
}

impl_99!();