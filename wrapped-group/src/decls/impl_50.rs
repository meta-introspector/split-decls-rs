macro_rules! deps {
    () => {
        ScalarMulOwned!();
        ScalarMul!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T , Rhs , Output > ScalarMulOwned < Rhs , Output > for T where T : for < 'r > ScalarMul < & 'r Rhs , Output > { }
    };
}

impl_50!();