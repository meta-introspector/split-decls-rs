macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl Default for Inner { fn default () -> Self { Self :: from_static_ref (std :: ffi :: OsStr :: new ("")) } }
    };
}

impl_138!();