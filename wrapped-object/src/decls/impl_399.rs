macro_rules! deps {
    () => {
        Dyn64!();
        Endian!();
        Dyn!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Dyn for elf :: Dyn64 < Endian > { type Word = u64 ; type Endian = Endian ; # [inline] fn d_tag (& self , endian : Self :: Endian) -> Self :: Word { self . d_tag . get (endian) } # [inline] fn d_val (& self , endian : Self :: Endian) -> Self :: Word { self . d_val . get (endian) } }
    };
}

impl_399!();