macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl core :: ops :: Sub < Self > for Matrix4x4 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self . impl_sub (& rhs) } }
    };
}

impl_37!()