macro_rules! deps {
    () => {
        Word!();
        Int!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsMut < [Word ; LIMBS] > for Int < LIMBS > { fn as_mut (& mut self) -> & mut [Word ; LIMBS] { self . as_mut_words () } }
    };
}

impl_101!()