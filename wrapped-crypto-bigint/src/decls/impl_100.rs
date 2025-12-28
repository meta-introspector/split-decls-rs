macro_rules! deps {
    () => {
        Int!();
        Word!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsRef < [Word ; LIMBS] > for Int < LIMBS > { fn as_ref (& self) -> & [Word ; LIMBS] { self . as_words () } }
    };
}

impl_100!()