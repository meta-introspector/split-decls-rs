macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl PartialEq < OsStr > for String { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (self . as_str () , other . as_os_str ()) } }
    };
}

impl_133!();