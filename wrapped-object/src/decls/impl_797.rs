macro_rules! deps {
    () => {
        ObjectSection!();
        SectionIndex!();
        RelocationIterator!();
        Result!();
        XcoffRelocationIterator!();
        RelocationMap!();
        CompressedData!();
        SectionKind!();
        FileHeader!();
        XcoffSection!();
        CompressedFileRange!();
        ReadRef!();
        SectionFlags!();
    };
}

macro_rules! impl_797 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > ObjectSection < 'data > for XcoffSection < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type RelocationIterator = XcoffRelocationIterator < 'data , 'file , Xcoff , R > ; fn index (& self) -> SectionIndex { self . index } fn address (& self) -> u64 { self . section . s_paddr () . into () } fn size (& self) -> u64 { self . section . s_size () . into () } fn align (& self) -> u64 { if let Some (aux_header) = self . file . aux_header { match self . kind () { SectionKind :: Text => aux_header . o_algntext () . into () , SectionKind :: Data => aux_header . o_algndata () . into () , _ => 4 , } } else { 4 } } fn file_range (& self) -> Option < (u64 , u64) > { self . section . file_range () } fn data (& self) -> Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } fn compressed_file_range (& self) -> Result < CompressedFileRange > { Ok (CompressedFileRange :: none (self . file_range ())) } fn compressed_data (& self) -> Result < CompressedData < 'data > > { self . data () . map (CompressedData :: none) } fn name_bytes (& self) -> read :: Result < & 'data [u8] > { Ok (self . section . name ()) } fn name (& self) -> read :: Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 XCOFF section name") } fn segment_name_bytes (& self) -> Result < Option < & [u8] > > { Ok (None) } fn segment_name (& self) -> Result < Option < & str > > { Ok (None) } fn kind (& self) -> SectionKind { let section_type = self . section . s_flags () as u16 ; if section_type & xcoff :: STYP_TEXT != 0 { SectionKind :: Text } else if section_type & xcoff :: STYP_DATA != 0 { SectionKind :: Data } else if section_type & xcoff :: STYP_TDATA != 0 { SectionKind :: Tls } else if section_type & xcoff :: STYP_BSS != 0 { SectionKind :: UninitializedData } else if section_type & xcoff :: STYP_TBSS != 0 { SectionKind :: UninitializedTls } else if section_type & (xcoff :: STYP_DEBUG | xcoff :: STYP_DWARF) != 0 { SectionKind :: Debug } else if section_type & (xcoff :: STYP_LOADER | xcoff :: STYP_OVRFLO) != 0 { SectionKind :: Metadata } else if section_type & (xcoff :: STYP_INFO | xcoff :: STYP_EXCEPT | xcoff :: STYP_PAD | xcoff :: STYP_TYPCHK) != 0 { SectionKind :: Other } else { SectionKind :: Unknown } } fn relocations (& self) -> Self :: RelocationIterator { let rel = self . xcoff_relocations () . unwrap_or (& []) ; XcoffRelocationIterator { file : self . file , relocations : rel . iter () , } } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } fn flags (& self) -> SectionFlags { SectionFlags :: Xcoff { s_flags : self . section . s_flags () , } } fn uncompressed_data (& self) -> Result < alloc :: borrow :: Cow < 'data , [u8] > > { self . compressed_data () ? . decompress () } }
    };
}

impl_797!();