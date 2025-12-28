macro_rules! deps {
    () => {
        IeeeFloat!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        float_common_impls ! (IeeeFloat < S >) ;
    };
}

macro_36!();