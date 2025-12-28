macro_rules! deps {
    () => {
        ImageBaseRelocation!();
        RelocBlock!();
    };
}

macro_rules! impl_1008 {
    () => {
        deps!();
        impl RelocBlock { fn size (& self) -> u32 { mem :: size_of :: < pe :: ImageBaseRelocation > () as u32 + self . count * mem :: size_of :: < u16 > () as u32 } }
    };
}

impl_1008!()