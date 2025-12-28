macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl From < OsStr > for std :: path :: PathBuf { fn from (name : OsStr) -> Self { std :: ffi :: OsString :: from (name) . into () } }
    };
}

impl_120!();