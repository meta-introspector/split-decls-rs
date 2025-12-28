macro_rules! deps {
    () => {
        Int!();
        ConstCtOption!();
        Uint!();
        NonZeroInt!();
        NonZero!();
        ConstChoice!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < const LIMBS : usize > NonZeroInt < LIMBS > { # [doc = " Creates a new non-zero integer in a const context."] # [doc = " Panics if the value is zero."] # [doc = ""] # [doc = " In future versions of Rust it should be possible to replace this with"] # [doc = " `NonZero::new(…).unwrap()`"] pub const fn new_unwrap (n : Int < LIMBS >) -> Self { ConstCtOption :: new (Self (n) , n . is_nonzero ()) . expect ("Invalid value: zero") } # [doc = " The sign and magnitude of this [`NonZeroInt`]."] pub const fn abs_sign (& self) -> (NonZero < Uint < LIMBS > > , ConstChoice) { let (abs , sign) = self . 0 . abs_sign () ; (NonZero (abs) , sign) } # [doc = " The magnitude of this [`NonZeroInt`]."] pub const fn abs (& self) -> NonZero < Uint < LIMBS > > { self . abs_sign () . 0 } }
    };
}

impl_190!()