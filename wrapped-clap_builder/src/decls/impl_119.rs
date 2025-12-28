macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl From < OsStr > for std :: ffi :: OsString { fn from (name : OsStr) -> Self { name . name . into_os_string () } }
    };
}

impl_119!()