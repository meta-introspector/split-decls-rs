macro_rules! deps {
    () => {
        CompressedData!();
        RelocationIterator!();
        SectionKind!();
        ImageNtHeaders!();
        PeRelocationIterator!();
        RelocationMap!();
        SectionFlags!();
        CompressedFileRange!();
        ObjectSection!();
        ReadRef!();
        SectionIndex!();
        Result!();
        PeSection!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > ObjectSection < 'data > for PeSection < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type RelocationIterator = PeRelocationIterator < 'data , 'file , R > ; # [inline] fn index (& self) -> SectionIndex { self . index } # [inline] fn address (& self) -> u64 { u64 :: from (self . section . virtual_address . get (LE)) . wrapping_add (self . file . common . image_base) } # [inline] fn size (& self) -> u64 { u64 :: from (self . section . virtual_size . get (LE)) } # [inline] fn align (& self) -> u64 { self . file . section_alignment () } # [inline] fn file_range (& self) -> Option < (u64 , u64) > { let (offset , size) = self . section . pe_file_range () ; if size == 0 { None } else { Some ((u64 :: from (offset) , u64 :: from (size))) } } fn data (& self) -> Result < & 'data [u8] > { self . section . pe_data (self . file . data) } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . data () ? , self . address () , address , size ,)) } # [inline] fn compressed_file_range (& self) -> Result < CompressedFileRange > { Ok (CompressedFileRange :: none (self . file_range ())) } # [inline] fn compressed_data (& self) -> Result < CompressedData < 'data > > { self . data () . map (CompressedData :: none) } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { self . section . name (self . file . common . symbols . strings ()) } # [inline] fn name (& self) -> Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 PE section name") } # [inline] fn segment_name_bytes (& self) -> Result < Option < & [u8] > > { Ok (None) } # [inline] fn segment_name (& self) -> Result < Option < & str > > { Ok (None) } # [inline] fn kind (& self) -> SectionKind { self . section . kind () } fn relocations (& self) -> PeRelocationIterator < 'data , 'file , R > { PeRelocationIterator (PhantomData) } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } fn flags (& self) -> SectionFlags { SectionFlags :: Coff { characteristics : self . section . characteristics . get (LE) , } } }
    };
}

impl_675!();