macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for Vector2 { type Output = Self ; fn mul (self , rhs : f32) -> Self { self . impl_mul_f32 (rhs) } }
    };
}

impl_67!();