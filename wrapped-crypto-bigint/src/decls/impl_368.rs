macro_rules! deps {
    () => {
        Word!();
        Uint!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsRef < [Word ; LIMBS] > for Uint < LIMBS > { fn as_ref (& self) -> & [Word ; LIMBS] { self . as_words () } }
    };
}

impl_368!();