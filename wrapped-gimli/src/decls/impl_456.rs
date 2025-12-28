macro_rules! deps {
    () => {
        RawLocListIter!();
        Encoding!();
        Reader!();
        LocListsFormat!();
        DebugLocListsIndex!();
        DebugAddr!();
        Result!();
        LocationListsOffset!();
        LocationLists!();
        DebugLocListsBase!();
        ReaderOffsetId!();
        SectionId!();
        DebugAddrBase!();
        LocListIter!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < R : Reader > LocationLists < R > { # [doc = " Iterate over the `LocationListEntry`s starting at the given offset."] # [doc = ""] # [doc = " The `unit_encoding` must match the compilation unit that the"] # [doc = " offset was contained in."] # [doc = ""] # [doc = " The `base_address` should be obtained from the `DW_AT_low_pc` attribute in the"] # [doc = " `DW_TAG_compile_unit` entry for the compilation unit that contains this location"] # [doc = " list."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn locations (& self , offset : LocationListsOffset < R :: Offset > , unit_encoding : Encoding , base_address : u64 , debug_addr : & DebugAddr < R > , debug_addr_base : DebugAddrBase < R :: Offset > ,) -> Result < LocListIter < R > > { Ok (LocListIter :: new (self . raw_locations (offset , unit_encoding) ? , base_address , debug_addr . clone () , debug_addr_base ,)) } # [doc = " Similar to `locations`, but with special handling for .dwo files."] # [doc = " This should only been used when this `LocationLists` was loaded from a"] # [doc = " .dwo file."] pub fn locations_dwo (& self , offset : LocationListsOffset < R :: Offset > , unit_encoding : Encoding , base_address : u64 , debug_addr : & DebugAddr < R > , debug_addr_base : DebugAddrBase < R :: Offset > ,) -> Result < LocListIter < R > > { Ok (LocListIter :: new (self . raw_locations_dwo (offset , unit_encoding) ? , base_address , debug_addr . clone () , debug_addr_base ,)) } # [doc = " Iterate over the raw `LocationListEntry`s starting at the given offset."] # [doc = ""] # [doc = " The `unit_encoding` must match the compilation unit that the"] # [doc = " offset was contained in."] # [doc = ""] # [doc = " This iterator does not perform any processing of the location entries,"] # [doc = " such as handling base addresses."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn raw_locations (& self , offset : LocationListsOffset < R :: Offset > , unit_encoding : Encoding ,) -> Result < RawLocListIter < R > > { let (mut input , format) = if unit_encoding . version <= 4 { (self . debug_loc . section . clone () , LocListsFormat :: Bare) } else { (self . debug_loclists . section . clone () , LocListsFormat :: Lle) } ; input . skip (offset . 0) ? ; Ok (RawLocListIter :: new (input , unit_encoding , format)) } # [doc = " Similar to `raw_locations`, but with special handling for .dwo files."] # [doc = " This should only been used when this `LocationLists` was loaded from a"] # [doc = " .dwo file."] pub fn raw_locations_dwo (& self , offset : LocationListsOffset < R :: Offset > , unit_encoding : Encoding ,) -> Result < RawLocListIter < R > > { let mut input = if unit_encoding . version <= 4 { self . debug_loc . section . clone () } else { self . debug_loclists . section . clone () } ; input . skip (offset . 0) ? ; Ok (RawLocListIter :: new (input , unit_encoding , LocListsFormat :: Lle ,)) } # [doc = " Returns the `.debug_loclists` offset at the given `base` and `index`."] # [doc = ""] # [doc = " The `base` must be the `DW_AT_loclists_base` value from the compilation unit DIE."] # [doc = " This is an offset that points to the first entry following the header."] # [doc = ""] # [doc = " The `index` is the value of a `DW_FORM_loclistx` attribute."] pub fn get_offset (& self , unit_encoding : Encoding , base : DebugLocListsBase < R :: Offset > , index : DebugLocListsIndex < R :: Offset > ,) -> Result < LocationListsOffset < R :: Offset > > { let format = unit_encoding . format ; let input = & mut self . debug_loclists . section . clone () ; input . skip (base . 0) ? ; input . skip (R :: Offset :: from_u64 (index . 0 . into_u64 () * u64 :: from (format . word_size ()) ,) ?) ? ; input . read_offset (format) . map (| x | LocationListsOffset (base . 0 + x)) } # [doc = " Call `Reader::lookup_offset_id` for each section, and return the first match."] pub fn lookup_offset_id (& self , id : ReaderOffsetId) -> Option < (SectionId , R :: Offset) > { self . debug_loc . lookup_offset_id (id) . or_else (| | self . debug_loclists . lookup_offset_id (id)) } }
    };
}

impl_456!()