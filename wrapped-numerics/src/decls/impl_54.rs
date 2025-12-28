macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl core :: ops :: Sub < & Self > for Vector2 { type Output = Self ; fn sub (self , rhs : & Self) -> Self { self . impl_sub (rhs) } }
    };
}

impl_54!();