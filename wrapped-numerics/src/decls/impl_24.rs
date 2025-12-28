macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl core :: ops :: Sub < & Matrix3x2 > for & Matrix3x2 { type Output = Matrix3x2 ; fn sub (self , rhs : & Matrix3x2) -> Matrix3x2 { self . impl_sub (rhs) } }
    };
}

impl_24!()