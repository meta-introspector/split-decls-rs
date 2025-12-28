macro_rules! deps {
    () => {
        DoubleFloat!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        float_common_impls ! (DoubleFloat < F >) ;
    };
}

macro_61!()