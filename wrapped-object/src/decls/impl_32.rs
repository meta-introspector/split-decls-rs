macro_rules! deps {
    () => {
        Endian!();
        BigEndian!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Endian for BigEndian { # [inline] fn from_big_endian (big_endian : bool) -> Option < Self > { if big_endian { Some (BigEndian) } else { None } } # [inline] fn is_big_endian (self) -> bool { true } }
    };
}

impl_32!()