macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl core :: ops :: Sub < Self > for Vector3 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . impl_sub (& rhs) } }
    };
}

impl_75!();