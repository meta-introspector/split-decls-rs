macro_rules! deps {
    () => {
        Uint!();
        Odd!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < const LIMBS : usize > PartialEq < Odd < Uint < LIMBS > > > for Uint < LIMBS > { fn eq (& self , other : & Odd < Uint < LIMBS > >) -> bool { self . eq (& other . 0) } }
    };
}

impl_238!();