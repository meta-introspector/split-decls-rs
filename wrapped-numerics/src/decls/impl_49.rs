macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl core :: ops :: Add < Self > for Vector2 { type Output = Self ; fn add (self , rhs : Self) -> Self { self . impl_add (& rhs) } }
    };
}

impl_49!()