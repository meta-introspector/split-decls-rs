macro_rules! deps {
    () => {
        SectionHeader!();
        Endian!();
        SectionHeader64!();
        FileHeader64!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > SectionHeader for elf :: SectionHeader64 < Endian > { type Word = u64 ; type Endian = Endian ; type Elf = elf :: FileHeader64 < Endian > ; # [inline] fn sh_name (& self , endian : Self :: Endian) -> u32 { self . sh_name . get (endian) } # [inline] fn sh_type (& self , endian : Self :: Endian) -> u32 { self . sh_type . get (endian) } # [inline] fn sh_flags (& self , endian : Self :: Endian) -> Self :: Word { self . sh_flags . get (endian) } # [inline] fn sh_addr (& self , endian : Self :: Endian) -> Self :: Word { self . sh_addr . get (endian) } # [inline] fn sh_offset (& self , endian : Self :: Endian) -> Self :: Word { self . sh_offset . get (endian) } # [inline] fn sh_size (& self , endian : Self :: Endian) -> Self :: Word { self . sh_size . get (endian) } # [inline] fn sh_link (& self , endian : Self :: Endian) -> u32 { self . sh_link . get (endian) } # [inline] fn sh_info (& self , endian : Self :: Endian) -> u32 { self . sh_info . get (endian) } # [inline] fn sh_addralign (& self , endian : Self :: Endian) -> Self :: Word { self . sh_addralign . get (endian) } # [inline] fn sh_entsize (& self , endian : Self :: Endian) -> Self :: Word { self . sh_entsize . get (endian) } }
    };
}

impl_319!();