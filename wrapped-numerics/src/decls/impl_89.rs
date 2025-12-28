macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for Vector3 { type Output = Self ; fn mul (self , rhs : f32) -> Self { self . impl_mul_f32 (rhs) } }
    };
}

impl_89!()