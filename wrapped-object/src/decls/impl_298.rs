macro_rules! deps {
    () => {
        ElfSegment!();
        FileHeader!();
        SegmentFlags!();
        ObjectSegment!();
        Result!();
        ReadRef!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ObjectSegment < 'data > for ElfSegment < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { # [inline] fn address (& self) -> u64 { self . segment . p_vaddr (self . file . endian) . into () } # [inline] fn size (& self) -> u64 { self . segment . p_memsz (self . file . endian) . into () } # [inline] fn align (& self) -> u64 { self . segment . p_align (self . file . endian) . into () } # [inline] fn file_range (& self) -> (u64 , u64) { self . segment . file_range (self . file . endian) } # [inline] fn data (& self) -> read :: Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> read :: Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } # [inline] fn name_bytes (& self) -> read :: Result < Option < & [u8] > > { Ok (None) } # [inline] fn name (& self) -> read :: Result < Option < & str > > { Ok (None) } # [inline] fn flags (& self) -> SegmentFlags { let p_flags = self . segment . p_flags (self . file . endian) ; SegmentFlags :: Elf { p_flags } } }
    };
}

impl_298!()