macro_rules! deps {
    () => {
        CoffRelocationIterator!();
        SectionIndex!();
        ReadRef!();
        CoffHeader!();
        SectionKind!();
        RelocationMap!();
        SectionFlags!();
        Result!();
        CompressedFileRange!();
        RelocationIterator!();
        CoffSection!();
        CompressedData!();
        ObjectSection!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectSection < 'data > for CoffSection < 'data , 'file , R , Coff > { type RelocationIterator = CoffRelocationIterator < 'data , 'file , R , Coff > ; # [inline] fn index (& self) -> SectionIndex { self . index } # [inline] fn address (& self) -> u64 { u64 :: from (self . section . virtual_address . get (LE)) } # [inline] fn size (& self) -> u64 { u64 :: from (self . section . size_of_raw_data . get (LE)) } # [inline] fn align (& self) -> u64 { self . section . coff_alignment () } # [inline] fn file_range (& self) -> Option < (u64 , u64) > { let (offset , size) = self . section . coff_file_range () ? ; Some ((u64 :: from (offset) , u64 :: from (size))) } fn data (& self) -> Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } # [inline] fn compressed_file_range (& self) -> Result < CompressedFileRange > { Ok (CompressedFileRange :: none (self . file_range ())) } # [inline] fn compressed_data (& self) -> Result < CompressedData < 'data > > { self . data () . map (CompressedData :: none) } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { self . section . name (self . file . common . symbols . strings ()) } # [inline] fn name (& self) -> Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 COFF section name") } # [inline] fn segment_name_bytes (& self) -> Result < Option < & [u8] > > { Ok (None) } # [inline] fn segment_name (& self) -> Result < Option < & str > > { Ok (None) } # [inline] fn kind (& self) -> SectionKind { self . section . kind () } fn relocations (& self) -> CoffRelocationIterator < 'data , 'file , R , Coff > { let relocations = self . coff_relocations () . unwrap_or (& []) ; CoffRelocationIterator { file : self . file , iter : relocations . iter () , } } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } fn flags (& self) -> SectionFlags { SectionFlags :: Coff { characteristics : self . section . characteristics . get (LE) , } } }
    };
}

impl_222!()