macro_rules! deps {
    () => {
        Error!();
        Reader!();
        Result!();
    };
}

macro_rules! parse_pointer_encoding {
    () => {
        deps!();
        # [doc = " Parse a `DW_EH_PE_*` pointer encoding."] # [doc (hidden)] # [inline] fn parse_pointer_encoding < R : Reader > (input : & mut R) -> Result < constants :: DwEhPe > { let eh_pe = input . read_u8 () ? ; let eh_pe = constants :: DwEhPe (eh_pe) ; if eh_pe . is_valid_encoding () { Ok (eh_pe) } else { Err (Error :: UnknownPointerEncoding (eh_pe)) } }
    };
}

parse_pointer_encoding!();