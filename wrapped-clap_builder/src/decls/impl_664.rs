macro_rules! deps {
    () => {
        Id!();
        Str!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        impl PartialEq < Id > for Str { # [inline] fn eq (& self , other : & Id) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
    };
}

impl_664!()