macro_rules! deps {
    () => {
        LittleEndian!();
        Endianity!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Endianity for LittleEndian { # [inline] fn is_big_endian (self) -> bool { false } }
    };
}

impl_106!()