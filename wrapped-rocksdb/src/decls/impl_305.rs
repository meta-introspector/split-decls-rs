macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl core :: convert :: AsRef < str > for PropName { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_305!();