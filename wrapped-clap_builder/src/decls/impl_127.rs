macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl PartialEq < OsStr > for str { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (self , other . as_os_str ()) } }
    };
}

impl_127!();