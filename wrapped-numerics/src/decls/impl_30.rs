macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for & Matrix3x2 { type Output = Matrix3x2 ; fn mul (self , rhs : f32) -> Matrix3x2 { self . impl_mul_f32 (rhs) } }
    };
}

impl_30!();