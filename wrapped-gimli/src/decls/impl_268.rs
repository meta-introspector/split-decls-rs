macro_rules! deps {
    () => {
        DebugTypes!();
        DebugInfo!();
        Error!();
        DebugTypeSignature!();
        DebugStrOffsets!();
        DebugMacinfo!();
        DebugLine!();
        DebugLoc!();
        DebugRngLists!();
        DwarfFileType!();
        DebugAbbrev!();
        AbbreviationsCache!();
        DwarfPackage!();
        DwarfPackageSections!();
        IndexSectionId!();
        RangeLists!();
        UnitIndexSectionIterator!();
        DebugMacro!();
        DebugLocLists!();
        Reader!();
        DwoId!();
        LocationLists!();
        Dwarf!();
        SectionId!();
        Result!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < R : Reader > DwarfPackage < R > { # [doc = " Try to load the `.dwp` sections using the given loader function."] # [doc = ""] # [doc = " `section` loads a DWARF section from the object file."] # [doc = " It should return an empty section if the section does not exist."] pub fn load < F , E > (section : F , empty : R) -> core :: result :: Result < Self , E > where F : FnMut (SectionId) -> core :: result :: Result < R , E > , E : From < Error > , { let sections = DwarfPackageSections :: load (section) ? ; Ok (Self :: from_sections (sections , empty) ?) } # [doc = " Create a `DwarfPackage` structure from the given sections."] fn from_sections (sections : DwarfPackageSections < R > , empty : R) -> Result < Self > { Ok (DwarfPackage { cu_index : sections . cu_index . index () ? , tu_index : sections . tu_index . index () ? , debug_abbrev : sections . debug_abbrev , debug_info : sections . debug_info , debug_line : sections . debug_line , debug_str : sections . debug_str , debug_str_offsets : sections . debug_str_offsets , debug_loc : sections . debug_loc , debug_loclists : sections . debug_loclists , debug_rnglists : sections . debug_rnglists , debug_types : sections . debug_types , empty , }) } # [doc = " Find the compilation unit with the given DWO identifier and return its section"] # [doc = " contributions."] # [doc = ""] # [doc = " ## Example Usage"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # fn example<R: gimli::Reader>("] # [doc = " #        dwarf: &gimli::Dwarf<R>,"] # [doc = " #        dwp: &gimli::DwarfPackage<R>,"] # [doc = " #        dwo_id: gimli::DwoId,"] # [doc = " # ) -> Result<(), gimli::Error> {"] # [doc = " if let Some(dwo) = dwp.find_cu(dwo_id, dwarf)? {"] # [doc = "    let dwo_header = dwo.units().next()?.expect(\"DWO should have one unit\");"] # [doc = "    let dwo_unit = dwo.unit(dwo_header)?;"] # [doc = "    // Do something with `dwo_unit`."] # [doc = " }"] # [doc = " # unreachable!()"] # [doc = " # }"] pub fn find_cu (& self , id : DwoId , parent : & Dwarf < R >) -> Result < Option < Dwarf < R > > > { let row = match self . cu_index . find (id . 0) { Some (row) => row , None => return Ok (None) , } ; self . cu_sections (row , parent) . map (Some) } # [doc = " Find the type unit with the given type signature and return its section"] # [doc = " contributions."] pub fn find_tu (& self , signature : DebugTypeSignature , parent : & Dwarf < R > ,) -> Result < Option < Dwarf < R > > > { let row = match self . tu_index . find (signature . 0) { Some (row) => row , None => return Ok (None) , } ; self . tu_sections (row , parent) . map (Some) } # [doc = " Return the section contributions of the compilation unit at the given index."] # [doc = ""] # [doc = " The index must be in the range `1..cu_index.unit_count`."] # [doc = ""] # [doc = " This function should only be needed by low level parsers."] pub fn cu_sections (& self , index : u32 , parent : & Dwarf < R >) -> Result < Dwarf < R > > { self . sections (self . cu_index . sections (index) ? , parent) } # [doc = " Return the section contributions of the compilation unit at the given index."] # [doc = ""] # [doc = " The index must be in the range `1..tu_index.unit_count`."] # [doc = ""] # [doc = " This function should only be needed by low level parsers."] pub fn tu_sections (& self , index : u32 , parent : & Dwarf < R >) -> Result < Dwarf < R > > { self . sections (self . tu_index . sections (index) ? , parent) } # [doc = " Return the section contributions of a unit."] # [doc = ""] # [doc = " This function should only be needed by low level parsers."] pub fn sections (& self , sections : UnitIndexSectionIterator < '_ , R > , parent : & Dwarf < R > ,) -> Result < Dwarf < R > > { let mut abbrev_offset = 0 ; let mut abbrev_size = 0 ; let mut info_offset = 0 ; let mut info_size = 0 ; let mut line_offset = 0 ; let mut line_size = 0 ; let mut loc_offset = 0 ; let mut loc_size = 0 ; let mut loclists_offset = 0 ; let mut loclists_size = 0 ; let mut str_offsets_offset = 0 ; let mut str_offsets_size = 0 ; let mut rnglists_offset = 0 ; let mut rnglists_size = 0 ; let mut types_offset = 0 ; let mut types_size = 0 ; for section in sections { match section . section { IndexSectionId :: DebugAbbrev => { abbrev_offset = section . offset ; abbrev_size = section . size ; } IndexSectionId :: DebugInfo => { info_offset = section . offset ; info_size = section . size ; } IndexSectionId :: DebugLine => { line_offset = section . offset ; line_size = section . size ; } IndexSectionId :: DebugLoc => { loc_offset = section . offset ; loc_size = section . size ; } IndexSectionId :: DebugLocLists => { loclists_offset = section . offset ; loclists_size = section . size ; } IndexSectionId :: DebugStrOffsets => { str_offsets_offset = section . offset ; str_offsets_size = section . size ; } IndexSectionId :: DebugRngLists => { rnglists_offset = section . offset ; rnglists_size = section . size ; } IndexSectionId :: DebugTypes => { types_offset = section . offset ; types_size = section . size ; } IndexSectionId :: DebugMacro | IndexSectionId :: DebugMacinfo => { } } } let debug_abbrev = self . debug_abbrev . dwp_range (abbrev_offset , abbrev_size) ? ; let debug_info = self . debug_info . dwp_range (info_offset , info_size) ? ; let debug_line = self . debug_line . dwp_range (line_offset , line_size) ? ; let debug_loc = self . debug_loc . dwp_range (loc_offset , loc_size) ? ; let debug_loclists = self . debug_loclists . dwp_range (loclists_offset , loclists_size) ? ; let debug_str_offsets = self . debug_str_offsets . dwp_range (str_offsets_offset , str_offsets_size) ? ; let debug_rnglists = self . debug_rnglists . dwp_range (rnglists_offset , rnglists_size) ? ; let debug_types = self . debug_types . dwp_range (types_offset , types_size) ? ; let debug_str = self . debug_str . clone () ; let debug_addr = parent . debug_addr . clone () ; let debug_ranges = parent . ranges . debug_ranges () . clone () ; let debug_aranges = self . empty . clone () . into () ; let debug_line_str = self . empty . clone () . into () ; let debug_macinfo = self . empty . clone () . into () ; let debug_macro = self . empty . clone () . into () ; Ok (Dwarf { debug_abbrev , debug_addr , debug_aranges , debug_info , debug_line , debug_line_str , debug_macinfo , debug_macro , debug_str , debug_str_offsets , debug_types , locations : LocationLists :: new (debug_loc , debug_loclists) , ranges : RangeLists :: new (debug_ranges , debug_rnglists) , file_type : DwarfFileType :: Dwo , sup : parent . sup . clone () , abbreviations_cache : AbbreviationsCache :: new () , }) } }
    };
}

impl_268!()