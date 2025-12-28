macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl std :: ops :: Deref for OsStr { type Target = std :: ffi :: OsStr ; # [inline] fn deref (& self) -> & std :: ffi :: OsStr { self . as_os_str () } }
    };
}

impl_122!();