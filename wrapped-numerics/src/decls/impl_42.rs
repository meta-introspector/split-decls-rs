macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl core :: ops :: Mul < & Self > for Matrix4x4 { type Output = Self ; fn mul (self , rhs : & Self) -> Self { self . impl_mul (rhs) } }
    };
}

impl_42!();