macro_rules! deps {
    () => {
        Endian!();
        LittleEndian!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Endian for LittleEndian { # [inline] fn from_big_endian (big_endian : bool) -> Option < Self > { if big_endian { None } else { Some (LittleEndian) } } # [inline] fn is_big_endian (self) -> bool { false } }
    };
}

impl_29!()