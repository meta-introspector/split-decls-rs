macro_rules! deps {
    () => {
        Uint!();
        Word!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsMut < [Word ; LIMBS] > for Uint < LIMBS > { fn as_mut (& mut self) -> & mut [Word ; LIMBS] { self . as_mut_words () } }
    };
}

impl_369!()