macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl PartialEq < String > for Str { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
    };
}

impl_220!()