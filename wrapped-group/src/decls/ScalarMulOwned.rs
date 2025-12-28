macro_rules! deps {
    () => {
        ScalarMul!();
    };
}

macro_rules! ScalarMulOwned {
    () => {
        deps!();
        # [doc = " A helper trait for references implementing group scalar multiplication."] pub trait ScalarMulOwned < Rhs , Output = Self > : for < 'r > ScalarMul < & 'r Rhs , Output > { }
    };
}

ScalarMulOwned!()