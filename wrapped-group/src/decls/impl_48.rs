macro_rules! deps {
    () => {
        ScalarMul!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T , Rhs , Output > ScalarMul < Rhs , Output > for T where T : Mul < Rhs , Output = Output > + MulAssign < Rhs > { }
    };
}

impl_48!();