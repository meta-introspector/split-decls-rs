macro_rules! deps {
    () => {
        RawRangeListsOffset!();
        DebuggingInformationEntry!();
        Reader!();
        AttributeValue!();
        LocListIter!();
        Dwarf!();
        LocationListsOffset!();
        DebugStrOffsetsIndex!();
        DebugLineStrOffset!();
        RawRngListIter!();
        DebugMacinfoOffset!();
        RangeListsOffset!();
        RawLocListIter!();
        MacroIter!();
        DebugMacroOffset!();
        RngListIter!();
        UnitRef!();
        DebugLocListsIndex!();
        Unit!();
        DebugStrOffset!();
        DebugRngListsIndex!();
        RangeIter!();
        DebugAddrIndex!();
        Result!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'a , R : Reader > UnitRef < 'a , R > { # [doc = " Construct a new `UnitRef` from a `Dwarf` and a `Unit`."] pub fn new (dwarf : & 'a Dwarf < R > , unit : & 'a Unit < R >) -> Self { UnitRef { dwarf , unit } } # [doc = " Return the string offset at the given index."] # [inline] pub fn string_offset (& self , index : DebugStrOffsetsIndex < R :: Offset > ,) -> Result < DebugStrOffset < R :: Offset > > { self . dwarf . string_offset (self . unit , index) } # [doc = " Return the string at the given offset in `.debug_str`."] # [inline] pub fn string (& self , offset : DebugStrOffset < R :: Offset >) -> Result < R > { self . dwarf . string (offset) } # [doc = " Return the string at the given offset in `.debug_line_str`."] # [inline] pub fn line_string (& self , offset : DebugLineStrOffset < R :: Offset >) -> Result < R > { self . dwarf . line_string (offset) } # [doc = " Return the string at the given offset in the `.debug_str`"] # [doc = " in the supplementary object file."] # [inline] pub fn sup_string (& self , offset : DebugStrOffset < R :: Offset >) -> Result < R > { self . dwarf . sup_string (offset) } # [doc = " Return an attribute value as a string slice."] # [doc = ""] # [doc = " See [`Dwarf::attr_string`] for more information."] pub fn attr_string (& self , attr : AttributeValue < R >) -> Result < R > { self . dwarf . attr_string (self . unit , attr) } # [doc = " Return the address at the given index."] pub fn address (& self , index : DebugAddrIndex < R :: Offset >) -> Result < u64 > { self . dwarf . address (self . unit , index) } # [doc = " Try to return an attribute value as an address."] # [doc = ""] # [doc = " See [`Dwarf::attr_address`] for more information."] pub fn attr_address (& self , attr : AttributeValue < R >) -> Result < Option < u64 > > { self . dwarf . attr_address (self . unit , attr) } # [doc = " Return the range list offset for the given raw offset."] # [doc = ""] # [doc = " This handles adding `DW_AT_GNU_ranges_base` if required."] pub fn ranges_offset_from_raw (& self , offset : RawRangeListsOffset < R :: Offset > ,) -> RangeListsOffset < R :: Offset > { self . dwarf . ranges_offset_from_raw (self . unit , offset) } # [doc = " Return the range list offset at the given index."] pub fn ranges_offset (& self , index : DebugRngListsIndex < R :: Offset > ,) -> Result < RangeListsOffset < R :: Offset > > { self . dwarf . ranges_offset (self . unit , index) } # [doc = " Iterate over the `RangeListEntry`s starting at the given offset."] pub fn ranges (& self , offset : RangeListsOffset < R :: Offset >) -> Result < RngListIter < R > > { self . dwarf . ranges (self . unit , offset) } # [doc = " Iterate over the `RawRngListEntry`ies starting at the given offset."] pub fn raw_ranges (& self , offset : RangeListsOffset < R :: Offset >) -> Result < RawRngListIter < R > > { self . dwarf . raw_ranges (self . unit , offset) } # [doc = " Try to return an attribute value as a range list offset."] # [doc = ""] # [doc = " See [`Dwarf::attr_ranges_offset`] for more information."] pub fn attr_ranges_offset (& self , attr : AttributeValue < R > ,) -> Result < Option < RangeListsOffset < R :: Offset > > > { self . dwarf . attr_ranges_offset (self . unit , attr) } # [doc = " Try to return an attribute value as a range list entry iterator."] # [doc = ""] # [doc = " See [`Dwarf::attr_ranges`] for more information."] pub fn attr_ranges (& self , attr : AttributeValue < R >) -> Result < Option < RngListIter < R > > > { self . dwarf . attr_ranges (self . unit , attr) } # [doc = " Return an iterator for the address ranges of a `DebuggingInformationEntry`."] # [doc = ""] # [doc = " This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges`."] pub fn die_ranges (& self , entry : & DebuggingInformationEntry < '_ , '_ , R >) -> Result < RangeIter < R > > { self . dwarf . die_ranges (self . unit , entry) } # [doc = " Return an iterator for the address ranges of the `Unit`."] # [doc = ""] # [doc = " This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges` of the"] # [doc = " root `DebuggingInformationEntry`."] pub fn unit_ranges (& self) -> Result < RangeIter < R > > { self . dwarf . unit_ranges (self . unit) } # [doc = " Return the location list offset at the given index."] pub fn locations_offset (& self , index : DebugLocListsIndex < R :: Offset > ,) -> Result < LocationListsOffset < R :: Offset > > { self . dwarf . locations_offset (self . unit , index) } # [doc = " Iterate over the `LocationListEntry`s starting at the given offset."] pub fn locations (& self , offset : LocationListsOffset < R :: Offset >) -> Result < LocListIter < R > > { self . dwarf . locations (self . unit , offset) } # [doc = " Iterate over the raw `LocationListEntry`s starting at the given offset."] pub fn raw_locations (& self , offset : LocationListsOffset < R :: Offset > ,) -> Result < RawLocListIter < R > > { self . dwarf . raw_locations (self . unit , offset) } # [doc = " Try to return an attribute value as a location list offset."] # [doc = ""] # [doc = " See [`Dwarf::attr_locations_offset`] for more information."] pub fn attr_locations_offset (& self , attr : AttributeValue < R > ,) -> Result < Option < LocationListsOffset < R :: Offset > > > { self . dwarf . attr_locations_offset (self . unit , attr) } # [doc = " Try to return an attribute value as a location list entry iterator."] # [doc = ""] # [doc = " See [`Dwarf::attr_locations`] for more information."] pub fn attr_locations (& self , attr : AttributeValue < R >) -> Result < Option < LocListIter < R > > > { self . dwarf . attr_locations (self . unit , attr) } # [doc = " Try to return an iterator for the list of macros at the given `.debug_macinfo` offset."] pub fn macinfo (& self , offset : DebugMacinfoOffset < R :: Offset >) -> Result < MacroIter < R > > { self . dwarf . macinfo (offset) } # [doc = " Try to return an iterator for the list of macros at the given `.debug_macro` offset."] pub fn macros (& self , offset : DebugMacroOffset < R :: Offset >) -> Result < MacroIter < R > > { self . dwarf . macros (offset) } }
    };
}

impl_276!()