macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl PartialEq < OsStr > for & '_ str { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (* self , other . as_os_str ()) } }
    };
}

impl_129!();