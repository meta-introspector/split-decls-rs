macro_rules! deps {
    () => {
        Relr!();
        Relr32!();
        Endian!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Relr for elf :: Relr32 < Endian > { type Word = u32 ; type Endian = Endian ; const COUNT : u8 = 31 ; fn get (& self , endian : Self :: Endian) -> Self :: Word { self . 0 . get (endian) } fn next (offset : & mut Self :: Word , bits : & mut Self :: Word) -> Option < Self :: Word > { * offset += 4 ; * bits >>= 1 ; if * bits & 1 != 0 { Some (* offset) } else { None } } }
    };
}

impl_371!();