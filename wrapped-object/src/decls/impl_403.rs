macro_rules! deps {
    () => {
        CompressionHeader!();
        Endian!();
        CompressionHeader64!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > CompressionHeader for elf :: CompressionHeader64 < Endian > { type Word = u64 ; type Endian = Endian ; # [inline] fn ch_type (& self , endian : Self :: Endian) -> u32 { self . ch_type . get (endian) } # [inline] fn ch_size (& self , endian : Self :: Endian) -> Self :: Word { self . ch_size . get (endian) } # [inline] fn ch_addralign (& self , endian : Self :: Endian) -> Self :: Word { self . ch_addralign . get (endian) } }
    };
}

impl_403!();