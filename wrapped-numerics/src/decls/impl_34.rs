macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl core :: ops :: Add < & Self > for Matrix4x4 { type Output = Self ; fn add (self , rhs : & Self) -> Self { self . impl_add (rhs) } }
    };
}

impl_34!();