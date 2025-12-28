macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for & Vector3 { type Output = Vector3 ; fn mul (self , rhs : f32) -> Vector3 { self . impl_mul_f32 (rhs) } }
    };
}

impl_90!()