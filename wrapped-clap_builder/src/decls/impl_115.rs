macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl From < & '_ & 'static std :: ffi :: OsStr > for OsStr { fn from (name : & '_ & 'static std :: ffi :: OsStr) -> Self { Self :: from_static_ref (name) } }
    };
}

impl_115!();