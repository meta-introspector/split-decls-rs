macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl AsRef < OsStr > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & OsStr { self . as_os_str () } }
    };
}

impl_111!()