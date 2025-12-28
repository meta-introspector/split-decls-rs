macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl core :: convert :: AsRef < CStr > for PropertyName { # [inline] fn as_ref (& self) -> & CStr { self . as_c_str () } }
    };
}

impl_317!()