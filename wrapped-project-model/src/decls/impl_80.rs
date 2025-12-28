macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl AsRef < std :: ffi :: OsStr > for ManifestPath { fn as_ref (& self) -> & std :: ffi :: OsStr { self . file . as_ref () } }
    };
}

impl_80!()