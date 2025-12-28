macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < str > for PropName { # [inline] fn eq (& self , other : & str) -> bool { self . as_str () . eq (other) } }
    };
}

impl_310!();