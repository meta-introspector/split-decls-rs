macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Endian for Endianness { # [inline] fn from_big_endian (big_endian : bool) -> Option < Self > { Some (if big_endian { Endianness :: Big } else { Endianness :: Little }) } # [inline] fn is_big_endian (self) -> bool { self != Endianness :: Little } }
    };
}

impl_26!()