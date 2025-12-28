macro_rules! deps {
    () => {
        Rela!();
        Endian!();
        Rela32!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Rela for elf :: Rela32 < Endian > { type Word = u32 ; type Sword = i32 ; type Endian = Endian ; # [inline] fn r_offset (& self , endian : Self :: Endian) -> Self :: Word { self . r_offset . get (endian) } # [inline] fn r_info (& self , endian : Self :: Endian , _is_mips64el : bool) -> Self :: Word { self . r_info . get (endian) } # [inline] fn r_addend (& self , endian : Self :: Endian) -> Self :: Sword { self . r_addend . get (endian) } # [inline] fn r_sym (& self , endian : Self :: Endian , _is_mips64el : bool) -> u32 { self . r_sym (endian) } # [inline] fn r_type (& self , endian : Self :: Endian , _is_mips64el : bool) -> u32 { self . r_type (endian) } }
    };
}

impl_365!();