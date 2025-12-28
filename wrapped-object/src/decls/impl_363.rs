macro_rules! deps {
    () => {
        Rel!();
        Rel64!();
        Endian!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Rel for elf :: Rel64 < Endian > { type Word = u64 ; type Sword = i64 ; type Endian = Endian ; # [inline] fn r_offset (& self , endian : Self :: Endian) -> Self :: Word { self . r_offset . get (endian) } # [inline] fn r_info (& self , endian : Self :: Endian) -> Self :: Word { self . r_info . get (endian) } # [inline] fn r_sym (& self , endian : Self :: Endian) -> u32 { self . r_sym (endian) } # [inline] fn r_type (& self , endian : Self :: Endian) -> u32 { self . r_type (endian) } }
    };
}

impl_363!()