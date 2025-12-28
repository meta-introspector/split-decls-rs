macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for Matrix4x4 { type Output = Self ; fn mul (self , rhs : f32) -> Self { self . impl_mul_f32 (rhs) } }
    };
}

impl_45!()