macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl core :: ops :: Mul < Vector2 > for & Vector2 { type Output = Vector2 ; fn mul (self , rhs : Vector2) -> Vector2 { self . impl_mul (& rhs) } }
    };
}

impl_65!()