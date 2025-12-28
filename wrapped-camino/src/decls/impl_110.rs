macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl AsRef < OsStr > for Utf8Path { # [inline] fn as_ref (& self) -> & OsStr { self . as_os_str () } }
    };
}

impl_110!()