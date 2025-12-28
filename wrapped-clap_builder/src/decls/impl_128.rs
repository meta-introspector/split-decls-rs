macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for OsStr { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_os_str () , * other) } }
    };
}

impl_128!()