macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsRef < UintRef > for Uint < LIMBS > { fn as_ref (& self) -> & UintRef { self . as_uint_ref () } }
    };
}

impl_372!();