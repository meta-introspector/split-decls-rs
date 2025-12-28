macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl core :: ops :: Sub < & Self > for Vector4 { type Output = Self ; fn sub (self , rhs : & Self) -> Self { self . impl_sub (rhs) } }
    };
}

impl_98!();