macro_rules! deps {
    () => {
        Context!();
        Error!();
        SupUnits!();
        Result!();
        ResUnits!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < R : gimli :: Reader > Context < R > { # [doc = " Construct a new `Context` from DWARF sections."] # [doc = ""] # [doc = " This method does not support using a supplementary object file."] # [allow (clippy :: too_many_arguments)] pub fn from_sections (debug_abbrev : gimli :: DebugAbbrev < R > , debug_addr : gimli :: DebugAddr < R > , debug_aranges : gimli :: DebugAranges < R > , debug_info : gimli :: DebugInfo < R > , debug_line : gimli :: DebugLine < R > , debug_line_str : gimli :: DebugLineStr < R > , debug_ranges : gimli :: DebugRanges < R > , debug_rnglists : gimli :: DebugRngLists < R > , debug_str : gimli :: DebugStr < R > , debug_str_offsets : gimli :: DebugStrOffsets < R > , default_section : R ,) -> Result < Self , Error > { Self :: from_dwarf (gimli :: Dwarf { debug_abbrev , debug_addr , debug_aranges , debug_info , debug_line , debug_line_str , debug_macinfo : default_section . clone () . into () , debug_macro : default_section . clone () . into () , debug_str , debug_str_offsets , debug_types : default_section . clone () . into () , locations : gimli :: LocationLists :: new (default_section . clone () . into () , default_section . into () ,) , ranges : gimli :: RangeLists :: new (debug_ranges , debug_rnglists) , file_type : gimli :: DwarfFileType :: Main , sup : None , abbreviations_cache : gimli :: AbbreviationsCache :: new () , }) } # [doc = " Construct a new `Context` from an existing [`gimli::Dwarf`] object."] # [inline] pub fn from_dwarf (sections : gimli :: Dwarf < R >) -> Result < Context < R > , Error > { Self :: from_arc_dwarf (Arc :: new (sections)) } # [doc = " Construct a new `Context` from an existing [`gimli::Dwarf`] object."] # [inline] pub fn from_arc_dwarf (sections : Arc < gimli :: Dwarf < R > >) -> Result < Context < R > , Error > { let units = ResUnits :: parse (& sections) ? ; let sup_units = if let Some (sup) = sections . sup . as_ref () { SupUnits :: parse (sup) ? } else { SupUnits :: default () } ; Ok (Context { sections , units , sup_units , }) } }
    };
}

impl_97!();