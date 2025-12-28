macro_rules! deps {
    () => {
        ReadRef!();
        RelocationMap!();
        CompressedFileRange!();
        SectionIndex!();
        MachO!();
        MachHeader!();
        SectionKind!();
        SectionFlags!();
        Result!();
        MachORelocationIterator!();
        RelocationIterator!();
        MachOSection!();
        ObjectSection!();
        CompressedData!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > ObjectSection < 'data > for MachOSection < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type RelocationIterator = MachORelocationIterator < 'data , 'file , Mach , R > ; # [inline] fn index (& self) -> SectionIndex { self . internal . index } # [inline] fn address (& self) -> u64 { self . internal . section . addr (self . file . endian) . into () } # [inline] fn size (& self) -> u64 { self . internal . section . size (self . file . endian) . into () } # [inline] fn align (& self) -> u64 { let align = self . internal . section . align (self . file . endian) ; if align < 64 { 1 << align } else { 0 } } # [inline] fn file_range (& self) -> Option < (u64 , u64) > { self . internal . section . file_range (self . file . endian) } # [inline] fn data (& self) -> Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } fn compressed_file_range (& self) -> Result < CompressedFileRange > { Ok (if let Some (data) = self . maybe_compressed_gnu () ? { data } else { CompressedFileRange :: none (self . file_range ()) }) } fn compressed_data (& self) -> read :: Result < CompressedData < 'data > > { self . compressed_file_range () ? . data (self . file . data) } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { Ok (self . internal . section . name ()) } # [inline] fn name (& self) -> Result < & 'data str > { str :: from_utf8 (self . internal . section . name ()) . ok () . read_error ("Non UTF-8 Mach-O section name") } # [inline] fn segment_name_bytes (& self) -> Result < Option < & [u8] > > { Ok (Some (self . internal . section . segment_name ())) } # [inline] fn segment_name (& self) -> Result < Option < & str > > { Ok (Some (str :: from_utf8 (self . internal . section . segment_name ()) . ok () . read_error ("Non UTF-8 Mach-O segment name") ? ,)) } fn kind (& self) -> SectionKind { self . internal . kind } fn relocations (& self) -> MachORelocationIterator < 'data , 'file , Mach , R > { MachORelocationIterator { file : self . file , relocations : self . macho_relocations () . unwrap_or (& []) . iter () , } } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } fn flags (& self) -> SectionFlags { SectionFlags :: MachO { flags : self . internal . section . flags (self . file . endian) , } } }
    };
}

impl_590!()