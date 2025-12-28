macro_rules! deps {
    () => {
        BigEndian!();
        Endianity!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Endianity for BigEndian { # [inline] fn is_big_endian (self) -> bool { true } }
    };
}

impl_109!();