macro_rules! deps {
    () => {
        Utf8Components!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl AsRef < OsStr > for Utf8Components < '_ > { # [inline] fn as_ref (& self) -> & OsStr { self . as_path () . as_os_str () } }
    };
}

impl_45!();