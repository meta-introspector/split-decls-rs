macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl PartialEq < OsStr > for std :: ffi :: OsString { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (self . as_os_str () , other . as_os_str ()) } }
    };
}

impl_135!()