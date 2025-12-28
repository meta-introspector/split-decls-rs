macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl core :: ops :: Div < Self > for Vector3 { type Output = Self ; fn div (self , rhs : Self) -> Self { self . impl_div (& rhs) } }
    };
}

impl_79!();