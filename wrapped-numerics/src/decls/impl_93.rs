macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl core :: ops :: Add < Self > for Vector4 { type Output = Self ; fn add (self , rhs : Self) -> Self { self . impl_add (& rhs) } }
    };
}

impl_93!();