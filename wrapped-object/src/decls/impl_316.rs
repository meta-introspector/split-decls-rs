macro_rules! deps {
    () => {
        Result!();
        Note!();
        ElfSectionRelocationIterator!();
        ElfSection!();
        CompressedData!();
        SectionFlags!();
        SectionKind!();
        FileHeader!();
        RelocationMap!();
        ReadRef!();
        SectionIndex!();
        RelocationIterator!();
        CompressedFileRange!();
        ObjectSection!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ObjectSection < 'data > for ElfSection < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type RelocationIterator = ElfSectionRelocationIterator < 'data , 'file , Elf , R > ; # [inline] fn index (& self) -> SectionIndex { self . index } # [inline] fn address (& self) -> u64 { self . section . sh_addr (self . file . endian) . into () } # [inline] fn size (& self) -> u64 { self . section . sh_size (self . file . endian) . into () } # [inline] fn align (& self) -> u64 { self . section . sh_addralign (self . file . endian) . into () } # [inline] fn file_range (& self) -> Option < (u64 , u64) > { self . section . file_range (self . file . endian) } # [inline] fn data (& self) -> read :: Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> read :: Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } fn compressed_file_range (& self) -> read :: Result < CompressedFileRange > { Ok (if let Some (data) = self . maybe_compressed () ? { data } else if let Some (data) = self . maybe_compressed_gnu () ? { data } else { CompressedFileRange :: none (self . file_range ()) }) } fn compressed_data (& self) -> read :: Result < CompressedData < 'data > > { self . compressed_file_range () ? . data (self . file . data) } fn name_bytes (& self) -> read :: Result < & 'data [u8] > { self . file . sections . section_name (self . file . endian , self . section) } fn name (& self) -> read :: Result < & 'data str > { let name = self . name_bytes () ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 ELF section name") } # [inline] fn segment_name_bytes (& self) -> read :: Result < Option < & [u8] > > { Ok (None) } # [inline] fn segment_name (& self) -> read :: Result < Option < & str > > { Ok (None) } fn kind (& self) -> SectionKind { let flags = self . section . sh_flags (self . file . endian) . into () ; let sh_type = self . section . sh_type (self . file . endian) ; match sh_type { elf :: SHT_PROGBITS => { if flags & u64 :: from (elf :: SHF_ALLOC) != 0 { if flags & u64 :: from (elf :: SHF_EXECINSTR) != 0 { SectionKind :: Text } else if flags & u64 :: from (elf :: SHF_TLS) != 0 { SectionKind :: Tls } else if flags & u64 :: from (elf :: SHF_WRITE) != 0 { SectionKind :: Data } else if flags & u64 :: from (elf :: SHF_STRINGS) != 0 { SectionKind :: ReadOnlyString } else { SectionKind :: ReadOnlyData } } else if flags & u64 :: from (elf :: SHF_STRINGS) != 0 { SectionKind :: OtherString } else { SectionKind :: Other } } elf :: SHT_NOBITS => { if flags & u64 :: from (elf :: SHF_TLS) != 0 { SectionKind :: UninitializedTls } else { SectionKind :: UninitializedData } } elf :: SHT_NOTE => SectionKind :: Note , elf :: SHT_NULL | elf :: SHT_SYMTAB | elf :: SHT_STRTAB | elf :: SHT_RELA | elf :: SHT_HASH | elf :: SHT_DYNAMIC | elf :: SHT_REL | elf :: SHT_DYNSYM | elf :: SHT_GROUP | elf :: SHT_SYMTAB_SHNDX | elf :: SHT_RELR | elf :: SHT_CREL => SectionKind :: Metadata , _ => SectionKind :: Elf (sh_type) , } } fn relocations (& self) -> ElfSectionRelocationIterator < 'data , 'file , Elf , R > { ElfSectionRelocationIterator { section_index : self . index , file : self . file , relocations : None , } } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } fn flags (& self) -> SectionFlags { SectionFlags :: Elf { sh_flags : self . section . sh_flags (self . file . endian) . into () , } } }
    };
}

impl_316!()