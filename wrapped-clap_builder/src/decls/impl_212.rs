macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl PartialEq < str > for Str { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . as_str () , other) } }
    };
}

impl_212!();