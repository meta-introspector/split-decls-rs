macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl core :: ops :: Mul < Matrix3x2 > for & Matrix3x2 { type Output = Matrix3x2 ; fn mul (self , rhs : Matrix3x2) -> Matrix3x2 { self . impl_mul (& rhs) } }
    };
}

impl_27!();