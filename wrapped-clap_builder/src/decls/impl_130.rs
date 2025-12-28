macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl PartialEq < & '_ std :: ffi :: OsStr > for OsStr { # [inline] fn eq (& self , other : & & std :: ffi :: OsStr) -> bool { PartialEq :: eq (self . as_os_str () , * other) } }
    };
}

impl_130!();