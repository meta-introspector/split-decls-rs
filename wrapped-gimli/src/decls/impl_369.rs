macro_rules! deps {
    () => {
        Error!();
        Encoding!();
        DebugInfoOffset!();
        DebugArangesOffset!();
        ReaderOffset!();
        ArangeHeader!();
        ArangeEntryIter!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < R , Offset > ArangeHeader < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn parse (input : & mut R , offset : DebugArangesOffset < Offset >) -> Result < Self > { let (length , format) = input . read_initial_length () ? ; let mut rest = input . split (length) ? ; let version = rest . read_u16 () ? ; if version != 2 && version != 3 { return Err (Error :: UnknownVersion (u64 :: from (version))) ; } let debug_info_offset = rest . read_offset (format) . map (DebugInfoOffset) ? ; let address_size = rest . read_address_size () ? ; let segment_size = rest . read_u8 () ? ; if segment_size != 0 { return Err (Error :: UnsupportedSegmentSize) ; } let header_length = format . initial_length_size () + 2 + format . word_size () + 1 + 1 ; let tuple_length = address_size . checked_mul (2) . ok_or (Error :: UnsupportedAddressSize (address_size)) ? ; if tuple_length == 0 { return Err (Error :: UnsupportedAddressSize (address_size)) ; } let padding = if header_length % tuple_length == 0 { 0 } else { tuple_length - header_length % tuple_length } ; rest . skip (R :: Offset :: from_u8 (padding)) ? ; let encoding = Encoding { format , version , address_size , } ; Ok (ArangeHeader { offset , encoding , length , debug_info_offset , entries : rest , }) } # [doc = " Return the offset of this header within the `.debug_aranges` section."] # [inline] pub fn offset (& self) -> DebugArangesOffset < Offset > { self . offset } # [doc = " Return the length of this set of entries, including the header."] # [inline] pub fn length (& self) -> Offset { self . length } # [doc = " Return the encoding parameters for this set of entries."] # [inline] pub fn encoding (& self) -> Encoding { self . encoding } # [doc = " Return the offset into the .debug_info section for this set of arange entries."] # [inline] pub fn debug_info_offset (& self) -> DebugInfoOffset < Offset > { self . debug_info_offset } # [doc = " Return the arange entries in this set."] # [inline] pub fn entries (& self) -> ArangeEntryIter < R > { ArangeEntryIter { input : self . entries . clone () , encoding : self . encoding , } } }
    };
}

impl_369!();