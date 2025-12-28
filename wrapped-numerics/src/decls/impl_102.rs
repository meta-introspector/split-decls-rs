macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl core :: ops :: Div < & Self > for Vector4 { type Output = Self ; fn div (self , rhs : & Self) -> Self { self . impl_div (rhs) } }
    };
}

impl_102!();