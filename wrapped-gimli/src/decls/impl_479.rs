macro_rules! deps {
    () => {
        PubStuffEntry!();
        PubStuffParser!();
        Error!();
        Reader!();
        LookupParser!();
        UnitOffset!();
        Result!();
        PubStuffHeader!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl < R , Entry > LookupParser < R > for PubStuffParser < R , Entry > where R : Reader , Entry : PubStuffEntry < R > , { type Header = PubStuffHeader < R :: Offset > ; type Entry = Entry ; # [doc = " Parse an pubthings set header. Returns a tuple of the"] # [doc = " pubthings to be parsed for this set, and the newly created PubThingHeader struct."] fn parse_header (input : & mut R) -> Result < (R , Self :: Header) > { let (length , format) = input . read_initial_length () ? ; let mut rest = input . split (length) ? ; let version = rest . read_u16 () ? ; if version != 2 { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let unit_offset = parse_debug_info_offset (& mut rest , format) ? ; let unit_length = rest . read_length (format) ? ; let header = PubStuffHeader { format , length , version , unit_offset , unit_length , } ; Ok ((rest , header)) } # [doc = " Parse a single pubthing. Return `None` for the null pubthing, `Some` for an actual pubthing."] fn parse_entry (input : & mut R , header : & Self :: Header) -> Result < Option < Self :: Entry > > { let offset = input . read_offset (header . format) ? ; if offset . into_u64 () == 0 { input . empty () ; Ok (None) } else { let name = input . read_null_terminated_slice () ? ; Ok (Some (Self :: Entry :: new (UnitOffset (offset) , name , header . unit_offset ,))) } } }
    };
}

impl_479!();