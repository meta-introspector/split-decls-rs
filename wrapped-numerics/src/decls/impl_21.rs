macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl core :: ops :: Sub < Self > for Matrix3x2 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . impl_sub (& rhs) } }
    };
}

impl_21!();