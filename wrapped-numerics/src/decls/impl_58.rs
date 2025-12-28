macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl core :: ops :: Div < & Self > for Vector2 { type Output = Self ; fn div (self , rhs : & Self) -> Self { self . impl_div (rhs) } }
    };
}

impl_58!();