macro_rules! deps {
    () => {
        ProgramHeader64!();
        FileHeader64!();
        Endian!();
        ProgramHeader!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > ProgramHeader for elf :: ProgramHeader64 < Endian > { type Word = u64 ; type Endian = Endian ; type Elf = elf :: FileHeader64 < Endian > ; # [inline] fn p_type (& self , endian : Self :: Endian) -> u32 { self . p_type . get (endian) } # [inline] fn p_flags (& self , endian : Self :: Endian) -> u32 { self . p_flags . get (endian) } # [inline] fn p_offset (& self , endian : Self :: Endian) -> Self :: Word { self . p_offset . get (endian) } # [inline] fn p_vaddr (& self , endian : Self :: Endian) -> Self :: Word { self . p_vaddr . get (endian) } # [inline] fn p_paddr (& self , endian : Self :: Endian) -> Self :: Word { self . p_paddr . get (endian) } # [inline] fn p_filesz (& self , endian : Self :: Endian) -> Self :: Word { self . p_filesz . get (endian) } # [inline] fn p_memsz (& self , endian : Self :: Endian) -> Self :: Word { self . p_memsz . get (endian) } # [inline] fn p_align (& self , endian : Self :: Endian) -> Self :: Word { self . p_align . get (endian) } }
    };
}

impl_301!()