macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl PartialEq < Str > for String { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
    };
}

impl_221!()