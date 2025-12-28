macro_rules! deps {
    () => {
        AddrEntryIter!();
        AddrHeader!();
        Encoding!();
        DebugAddrOffset!();
        Reader!();
        Result!();
        Error!();
        ReaderOffset!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < R , Offset > AddrHeader < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn parse (input : & mut R , offset : DebugAddrOffset < Offset >) -> Result < Self > { let (length , format) = input . read_initial_length () ? ; let mut rest = input . split (length) ? ; let version = rest . read_u16 () ? ; if version != 5 { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let address_size = rest . read_address_size () ? ; let segment_size = rest . read_u8 () ? ; if segment_size != 0 { return Err (Error :: UnsupportedSegmentSize) ; } let header_length = format . initial_length_size () + 2 + 1 + 1 ; let tuple_length = address_size ; if tuple_length == 0 { return Err (Error :: UnsupportedAddressSize (address_size)) ; } let padding = if header_length % tuple_length == 0 { 0 } else { tuple_length - header_length % tuple_length } ; rest . skip (R :: Offset :: from_u8 (padding)) ? ; let encoding = Encoding { format , version , address_size , } ; Ok (AddrHeader { offset , encoding , length , entries : rest , }) } # [doc = " Return the offset of this header within the `.debug_addr` section."] # [inline] pub fn offset (& self) -> DebugAddrOffset < Offset > { self . offset } # [doc = " Return the length of this set of entries, including the header."] # [inline] pub fn length (& self) -> Offset { self . length } # [doc = " Return the encoding parameters for this set of entries."] # [inline] pub fn encoding (& self) -> Encoding { self . encoding } # [doc = " Return the address entries in this set."] # [inline] pub fn entries (& self) -> AddrEntryIter < R > { AddrEntryIter { input : self . entries . clone () , encoding : self . encoding , } } }
    };
}

impl_152!()