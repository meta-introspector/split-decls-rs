macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl PartialEq < String > for OsStr { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . as_os_str () , other . as_str ()) } }
    };
}

impl_132!()