macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < & '_ std :: ffi :: OsString > for OsStr { fn from (name : & '_ std :: ffi :: OsString) -> Self { Self :: from_ref (name . as_os_str ()) } }
    };
}

impl_111!();