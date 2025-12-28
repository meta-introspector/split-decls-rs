macro_rules! deps {
    () => {
        Str!();
        Id!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl PartialEq < Str > for Id { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
    };
}

impl_663!()