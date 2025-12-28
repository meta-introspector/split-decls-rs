macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for & Vector4 { type Output = Vector4 ; fn mul (self , rhs : f32) -> Vector4 { self . impl_mul_f32 (rhs) } }
    };
}

impl_112!();