macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl core :: ops :: Mul < & Self > for Vector3 { type Output = Self ; fn mul (self , rhs : & Self) -> Self { self . impl_mul (rhs) } }
    };
}

impl_86!();