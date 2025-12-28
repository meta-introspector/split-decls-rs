macro_rules! deps {
    () => {
        Nlist!();
        Endian!();
        Nlist32!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Nlist for macho :: Nlist32 < Endian > { type Word = u32 ; type Endian = Endian ; fn n_strx (& self , endian : Self :: Endian) -> u32 { self . n_strx . get (endian) } fn n_type (& self) -> u8 { self . n_type } fn n_sect (& self) -> u8 { self . n_sect } fn n_desc (& self , endian : Self :: Endian) -> u16 { self . n_desc . get (endian) } fn n_value (& self , endian : Self :: Endian) -> Self :: Word { self . n_value . get (endian) } }
    };
}

impl_618!();