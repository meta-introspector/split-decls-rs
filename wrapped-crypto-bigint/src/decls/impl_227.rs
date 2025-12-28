macro_rules! deps {
    () => {
        ConstChoice!();
        Odd!();
        Int!();
        Uint!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < const LIMBS : usize > Odd < Int < LIMBS > > { # [doc = " The sign and magnitude of this [`Odd<Int<{LIMBS}>>`]."] pub const fn abs_sign (& self) -> (Odd < Uint < LIMBS > > , ConstChoice) { let (abs , sgn) = Int :: abs_sign (self . as_ref ()) ; (Odd (abs) , sgn) } # [doc = " The magnitude of this [`Odd<Int<{LIMBS}>>`]."] pub const fn abs (& self) -> Odd < Uint < LIMBS > > { self . abs_sign () . 0 } }
    };
}

impl_227!();