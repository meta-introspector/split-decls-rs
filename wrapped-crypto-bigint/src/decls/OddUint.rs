macro_rules! deps {
    () => {
        Uint!();
        Odd!();
    };
}

macro_rules! OddUint {
    () => {
        deps!();
        # [doc = " Non-zero unsigned integer."] pub type OddUint < const LIMBS : usize > = Odd < Uint < LIMBS > > ;
    };
}

OddUint!()