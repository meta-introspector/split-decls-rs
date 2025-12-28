macro_rules! deps {
    () => {
        IeeeFloat!();
        X87DoubleExtendedS!();
    };
}

macro_rules! X87DoubleExtended {
    () => {
        deps!();
        # [doc = " 80-bit floating point number that uses IEEE extended precision semantics, as used"] # [doc = " by x87 `long double`."] pub type X87DoubleExtended = IeeeFloat < X87DoubleExtendedS > ;
    };
}

X87DoubleExtended!()