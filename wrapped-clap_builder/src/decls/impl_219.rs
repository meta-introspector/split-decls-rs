macro_rules! deps {
    () => {
        OsStr!();
        Str!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl PartialEq < Str > for & '_ std :: ffi :: OsStr { # [inline] fn eq (& self , other : & Str) -> bool { PartialEq :: eq (* self , other . as_str ()) } }
    };
}

impl_219!()