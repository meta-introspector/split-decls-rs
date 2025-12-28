macro_rules! deps {
    () => {
        Sym!();
        Endian!();
        Sym32!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Sym for elf :: Sym32 < Endian > { type Word = u32 ; type Endian = Endian ; # [inline] fn st_name (& self , endian : Self :: Endian) -> u32 { self . st_name . get (endian) } # [inline] fn st_info (& self) -> u8 { self . st_info } # [inline] fn st_bind (& self) -> u8 { self . st_bind () } # [inline] fn st_type (& self) -> u8 { self . st_type () } # [inline] fn st_other (& self) -> u8 { self . st_other } # [inline] fn st_visibility (& self) -> u8 { self . st_visibility () } # [inline] fn st_shndx (& self , endian : Self :: Endian) -> u16 { self . st_shndx . get (endian) } # [inline] fn st_value (& self , endian : Self :: Endian) -> Self :: Word { self . st_value . get (endian) } # [inline] fn st_size (& self , endian : Self :: Endian) -> Self :: Word { self . st_size . get (endian) } }
    };
}

impl_342!()