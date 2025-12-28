macro_rules! deps {
    () => {
        Error!();
        Reader!();
        Result!();
        ListsHeader!();
        Encoding!();
    };
}

macro_rules! parse_header {
    () => {
        deps!();
        # [allow (dead_code)] fn parse_header < R : Reader > (input : & mut R) -> Result < ListsHeader > { let (length , format) = input . read_initial_length () ? ; input . truncate (length) ? ; let version = input . read_u16 () ? ; if version != 5 { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let address_size = input . read_address_size () ? ; let segment_selector_size = input . read_u8 () ? ; if segment_selector_size != 0 { return Err (Error :: UnsupportedSegmentSize) ; } let offset_entry_count = input . read_u32 () ? ; let encoding = Encoding { format , version , address_size , } ; Ok (ListsHeader { encoding , offset_entry_count , }) }
    };
}

parse_header!()