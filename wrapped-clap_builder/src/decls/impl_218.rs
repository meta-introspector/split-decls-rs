macro_rules! deps {
    () => {
        OsStr!();
        Str!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl PartialEq < & '_ std :: ffi :: OsStr > for Str { # [inline] fn eq (& self , other : & & std :: ffi :: OsStr) -> bool { PartialEq :: eq (self . as_str () , * other) } }
    };
}

impl_218!();