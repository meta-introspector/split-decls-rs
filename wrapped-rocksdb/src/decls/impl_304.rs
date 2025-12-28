macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl core :: convert :: AsRef < CStr > for PropName { # [inline] fn as_ref (& self) -> & CStr { self . as_c_str () } }
    };
}

impl_304!();