macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl core :: ops :: Mul < Self > for Vector2 { type Output = Self ; fn mul (self , rhs : Self) -> Self { self . impl_mul (& rhs) } }
    };
}

impl_63!()