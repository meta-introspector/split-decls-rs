macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl PartialEq < str > for OsStr { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . as_os_str () , other) } }
    };
}

impl_126!();