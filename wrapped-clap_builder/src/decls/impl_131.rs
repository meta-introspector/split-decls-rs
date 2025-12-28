macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl PartialEq < OsStr > for & '_ std :: ffi :: OsStr { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (* self , other . as_os_str ()) } }
    };
}

impl_131!();