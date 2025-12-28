macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl PartialEq < Id > for & '_ str { # [inline] fn eq (& self , other : & Id) -> bool { PartialEq :: eq (* self , other . as_str ()) } }
    };
}

impl_662!()