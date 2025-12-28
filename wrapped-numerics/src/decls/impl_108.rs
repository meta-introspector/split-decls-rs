macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl core :: ops :: Mul < & Self > for Vector4 { type Output = Self ; fn mul (self , rhs : & Self) -> Self { self . impl_mul (rhs) } }
    };
}

impl_108!();