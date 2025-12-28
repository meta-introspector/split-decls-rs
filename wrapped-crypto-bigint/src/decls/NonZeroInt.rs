macro_rules! deps {
    () => {
        Int!();
        NonZero!();
    };
}

macro_rules! NonZeroInt {
    () => {
        deps!();
        # [doc = " Non-zero signed integer."] pub type NonZeroInt < const LIMBS : usize > = NonZero < Int < LIMBS > > ;
    };
}

NonZeroInt!();