macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < std :: ffi :: OsStr > for OsStr { # [inline] fn borrow (& self) -> & std :: ffi :: OsStr { self . as_os_str () } }
    };
}

impl_125!()