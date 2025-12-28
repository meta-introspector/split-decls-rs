macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl AsRef < std :: ffi :: OsStr > for OsStr { # [inline] fn as_ref (& self) -> & std :: ffi :: OsStr { self . as_os_str () } }
    };
}

impl_123!()