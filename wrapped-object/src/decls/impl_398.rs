macro_rules! deps {
    () => {
        Endian!();
        Dyn!();
        Dyn32!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Dyn for elf :: Dyn32 < Endian > { type Word = u32 ; type Endian = Endian ; # [inline] fn d_tag (& self , endian : Self :: Endian) -> Self :: Word { self . d_tag . get (endian) } # [inline] fn d_val (& self , endian : Self :: Endian) -> Self :: Word { self . d_val . get (endian) } }
    };
}

impl_398!()