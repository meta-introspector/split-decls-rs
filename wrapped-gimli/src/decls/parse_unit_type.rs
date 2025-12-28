macro_rules! deps {
    () => {
        Reader!();
        Result!();
    };
}

macro_rules! parse_unit_type {
    () => {
        deps!();
        # [doc = " Parse the unit type from the unit header."] fn parse_unit_type < R : Reader > (input : & mut R) -> Result < constants :: DwUt > { let val = input . read_u8 () ? ; Ok (constants :: DwUt (val)) }
    };
}

parse_unit_type!()