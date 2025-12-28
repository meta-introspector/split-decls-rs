macro_rules! deps {
    () => {
        ScalarMul!();
        ScalarMulOwned!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T , Rhs , Output > ScalarMulOwned < Rhs , Output > for T where T : for < 'r > ScalarMul < & 'r Rhs , Output > { }
    };
}

impl_12!()