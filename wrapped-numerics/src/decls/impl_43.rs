macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl core :: ops :: Mul < Matrix4x4 > for & Matrix4x4 { type Output = Matrix4x4 ; fn mul (self , rhs : Matrix4x4) -> Matrix4x4 { self . impl_mul (& rhs) } }
    };
}

impl_43!();