macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < & 'static std :: ffi :: OsStr > for OsStr { fn from (name : & 'static std :: ffi :: OsStr) -> Self { Self :: from_static_ref (name) } }
    };
}

impl_114!();