macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl AsRef < str > for Id { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_657!();