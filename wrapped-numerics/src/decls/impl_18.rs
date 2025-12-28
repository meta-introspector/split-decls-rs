macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl core :: ops :: Add < & Self > for Matrix3x2 { type Output = Self ; fn add (self , rhs : & Self) -> Self { self . impl_add (rhs) } }
    };
}

impl_18!();