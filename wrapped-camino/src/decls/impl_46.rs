macro_rules! deps {
    () => {
        Utf8Component!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl AsRef < OsStr > for Utf8Component < '_ > { # [inline] fn as_ref (& self) -> & OsStr { self . as_os_str () } }
    };
}

impl_46!()