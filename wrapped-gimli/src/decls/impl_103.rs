macro_rules! deps {
    () => {
        RunTimeEndian!();
        Endianity!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Endianity for RunTimeEndian { # [inline] fn is_big_endian (self) -> bool { self != RunTimeEndian :: Little } }
    };
}

impl_103!();