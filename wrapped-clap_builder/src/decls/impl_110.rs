macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < std :: ffi :: OsString > for OsStr { fn from (name : std :: ffi :: OsString) -> Self { Self :: from_string (name) } }
    };
}

impl_110!();