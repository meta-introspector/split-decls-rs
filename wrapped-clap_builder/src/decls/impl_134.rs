macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl PartialEq < std :: ffi :: OsString > for OsStr { # [inline] fn eq (& self , other : & std :: ffi :: OsString) -> bool { PartialEq :: eq (self . as_os_str () , other . as_os_str ()) } }
    };
}

impl_134!();