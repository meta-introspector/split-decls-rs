macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl core :: ops :: Mul < Self > for Matrix3x2 { type Output = Self ; fn mul (self , rhs : Self) -> Self { self . impl_mul (& rhs) } }
    };
}

impl_25!()