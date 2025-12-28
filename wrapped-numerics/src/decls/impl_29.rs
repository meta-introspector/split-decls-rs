macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for Matrix3x2 { type Output = Self ; fn mul (self , rhs : f32) -> Self { self . impl_mul_f32 (rhs) } }
    };
}

impl_29!()