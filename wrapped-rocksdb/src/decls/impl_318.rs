macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl core :: convert :: AsRef < str > for PropertyName { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_318!()