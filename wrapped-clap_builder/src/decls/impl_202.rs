macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl From < Str > for std :: ffi :: OsString { fn from (name : Str) -> Self { String :: from (name) . into () } }
    };
}

impl_202!();