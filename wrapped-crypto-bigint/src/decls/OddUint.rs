macro_rules! deps {
    () => {
        Odd!();
        Uint!();
    };
}

macro_rules! OddUint {
    () => {
        deps!();
        # [doc = " Non-zero unsigned integer."] pub type OddUint < const LIMBS : usize > = Odd < Uint < LIMBS > > ;
    };
}

OddUint!();