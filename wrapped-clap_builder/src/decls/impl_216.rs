macro_rules! deps {
    () => {
        Str!();
        OsStr!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl PartialEq < std :: ffi :: OsStr > for Str { # [inline] fn eq (& self , other : & std :: ffi :: OsStr) -> bool { PartialEq :: eq (self . as_str () , other) } }
    };
}

impl_216!()