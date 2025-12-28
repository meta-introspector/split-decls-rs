macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl PartialEq < Str > for str { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (self , other . as_str ()) } }
    };
}

impl_213!();