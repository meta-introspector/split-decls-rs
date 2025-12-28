macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsMut < UintRef > for Uint < LIMBS > { fn as_mut (& mut self) -> & mut UintRef { self . as_mut_uint_ref () } }
    };
}

impl_373!();