macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl PartialEq < Str > for & '_ str { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (* self , other . as_str ()) } }
    };
}

impl_215!();