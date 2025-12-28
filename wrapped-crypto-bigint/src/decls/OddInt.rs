macro_rules! deps {
    () => {
        Odd!();
        Int!();
    };
}

macro_rules! OddInt {
    () => {
        deps!();
        # [doc = " Non-zero signed integer."] pub type OddInt < const LIMBS : usize > = Odd < Int < LIMBS > > ;
    };
}

OddInt!()