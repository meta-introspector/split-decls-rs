macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < CStr > for PropName { # [inline] fn eq (& self , other : & CStr) -> bool { self . as_c_str () . eq (other) } }
    };
}

impl_309!();