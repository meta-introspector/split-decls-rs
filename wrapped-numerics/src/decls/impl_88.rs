macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl core :: ops :: Mul < & Vector3 > for & Vector3 { type Output = Vector3 ; fn mul (self , rhs : & Vector3) -> Vector3 { self . impl_mul (rhs) } }
    };
}

impl_88!()