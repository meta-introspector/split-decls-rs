macro_rules! deps {
    () => {
        OsStr!();
        Str!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl PartialEq < Str > for std :: ffi :: OsStr { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (self , other . as_str ()) } }
    };
}

impl_217!();