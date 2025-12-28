macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl core :: ops :: Add < & Matrix3x2 > for & Matrix3x2 { type Output = Matrix3x2 ; fn add (self , rhs : & Matrix3x2) -> Matrix3x2 { self . impl_add (rhs) } }
    };
}

impl_20!();