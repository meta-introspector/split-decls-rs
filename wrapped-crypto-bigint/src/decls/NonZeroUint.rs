macro_rules! deps {
    () => {
        NonZero!();
        Uint!();
    };
}

macro_rules! NonZeroUint {
    () => {
        deps!();
        # [doc = " Non-zero unsigned integer."] pub type NonZeroUint < const LIMBS : usize > = NonZero < Uint < LIMBS > > ;
    };
}

NonZeroUint!()