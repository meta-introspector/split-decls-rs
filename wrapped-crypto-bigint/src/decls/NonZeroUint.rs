macro_rules! deps {
    () => {
        Uint!();
        NonZero!();
    };
}

macro_rules! NonZeroUint {
    () => {
        deps!();
        # [doc = " Non-zero unsigned integer."] pub type NonZeroUint < const LIMBS : usize > = NonZero < Uint < LIMBS > > ;
    };
}

NonZeroUint!();