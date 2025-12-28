macro_rules! deps {
    () => {
        Uint!();
        Odd!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < const LIMBS : usize > PartialOrd < Odd < Uint < LIMBS > > > for Uint < LIMBS > { fn partial_cmp (& self , other : & Odd < Uint < LIMBS > >) -> Option < Ordering > { Some (self . cmp (& other . 0)) } }
    };
}

impl_239!()