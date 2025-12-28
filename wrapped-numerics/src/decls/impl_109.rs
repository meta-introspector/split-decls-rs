macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl core :: ops :: Mul < Vector4 > for & Vector4 { type Output = Vector4 ; fn mul (self , rhs : Vector4) -> Vector4 { self . impl_mul (& rhs) } }
    };
}

impl_109!();