macro_rules! deps {
    () => {
        Str!();
        Id!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl From < Id > for Str { fn from (name : Id) -> Self { name . 0 } }
    };
}

impl_652!();