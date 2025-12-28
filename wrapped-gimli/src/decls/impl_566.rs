macro_rules! deps {
    () => {
        RangeListsOffset!();
        DebugRngListsIndex!();
        Reader!();
        RawRngListIter!();
        DebugRngListsBase!();
        RngListIter!();
        RangeListsFormat!();
        RangeLists!();
        DebugAddrBase!();
        Range!();
        Result!();
        ReaderOffsetId!();
        SectionId!();
        Encoding!();
        DebugAddr!();
    };
}

macro_rules! impl_566 {
    () => {
        deps!();
        impl < R : Reader > RangeLists < R > { # [doc = " Iterate over the `Range` list entries starting at the given offset."] # [doc = ""] # [doc = " The `unit_version` and `address_size` must match the compilation unit that the"] # [doc = " offset was contained in."] # [doc = ""] # [doc = " The `base_address` should be obtained from the `DW_AT_low_pc` attribute in the"] # [doc = " `DW_TAG_compile_unit` entry for the compilation unit that contains this range list."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn ranges (& self , offset : RangeListsOffset < R :: Offset > , unit_encoding : Encoding , base_address : u64 , debug_addr : & DebugAddr < R > , debug_addr_base : DebugAddrBase < R :: Offset > ,) -> Result < RngListIter < R > > { Ok (RngListIter :: new (self . raw_ranges (offset , unit_encoding) ? , base_address , debug_addr . clone () , debug_addr_base ,)) } # [doc = " Iterate over the `RawRngListEntry`ies starting at the given offset."] # [doc = ""] # [doc = " The `unit_encoding` must match the compilation unit that the"] # [doc = " offset was contained in."] # [doc = ""] # [doc = " This iterator does not perform any processing of the range entries,"] # [doc = " such as handling base addresses."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn raw_ranges (& self , offset : RangeListsOffset < R :: Offset > , unit_encoding : Encoding ,) -> Result < RawRngListIter < R > > { let (mut input , format) = if unit_encoding . version <= 4 { (self . debug_ranges . section . clone () , RangeListsFormat :: Bare) } else { (self . debug_rnglists . section . clone () , RangeListsFormat :: Rle) } ; input . skip (offset . 0) ? ; Ok (RawRngListIter :: new (input , unit_encoding , format)) } # [doc = " Returns the `.debug_rnglists` offset at the given `base` and `index`."] # [doc = ""] # [doc = " The `base` must be the `DW_AT_rnglists_base` value from the compilation unit DIE."] # [doc = " This is an offset that points to the first entry following the header."] # [doc = ""] # [doc = " The `index` is the value of a `DW_FORM_rnglistx` attribute."] # [doc = ""] # [doc = " The `unit_encoding` must match the compilation unit that the"] # [doc = " index was contained in."] pub fn get_offset (& self , unit_encoding : Encoding , base : DebugRngListsBase < R :: Offset > , index : DebugRngListsIndex < R :: Offset > ,) -> Result < RangeListsOffset < R :: Offset > > { let format = unit_encoding . format ; let input = & mut self . debug_rnglists . section . clone () ; input . skip (base . 0) ? ; input . skip (R :: Offset :: from_u64 (index . 0 . into_u64 () * u64 :: from (format . word_size ()) ,) ?) ? ; input . read_offset (format) . map (| x | RangeListsOffset (base . 0 + x)) } # [doc = " Call `Reader::lookup_offset_id` for each section, and return the first match."] pub fn lookup_offset_id (& self , id : ReaderOffsetId) -> Option < (SectionId , R :: Offset) > { self . debug_ranges . lookup_offset_id (id) . or_else (| | self . debug_rnglists . lookup_offset_id (id)) } }
    };
}

impl_566!();