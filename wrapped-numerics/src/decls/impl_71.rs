macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl core :: ops :: Add < Self > for Vector3 { type Output = Self ; fn add (self , rhs : Self) -> Self { self . impl_add (& rhs) } }
    };
}

impl_71!();