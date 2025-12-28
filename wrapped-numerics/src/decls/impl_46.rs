macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for & Matrix4x4 { type Output = Matrix4x4 ; fn mul (self , rhs : f32) -> Matrix4x4 { self . impl_mul_f32 (rhs) } }
    };
}

impl_46!()