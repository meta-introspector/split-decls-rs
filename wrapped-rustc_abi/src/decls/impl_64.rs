macro_rules! deps {
    () => {
        Endian!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Endian { pub fn as_str (& self) -> & 'static str { match self { Self :: Little => "little" , Self :: Big => "big" , } } }
    };
}

impl_64!()