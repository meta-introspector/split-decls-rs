macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for Vector4 { type Output = Self ; fn mul (self , rhs : f32) -> Self { self . impl_mul_f32 (rhs) } }
    };
}

impl_111!();