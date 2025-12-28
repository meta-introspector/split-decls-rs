macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        impl PartialEq < Id > for str { # [inline] fn eq (& self , other : & Id) -> bool { PartialEq :: eq (self , other . as_str ()) } }
    };
}

impl_660!();