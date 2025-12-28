macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl AsRef < OsStr > for Iter < '_ > { # [inline] fn as_ref (& self) -> & OsStr { self . as_path () . as_os_str () } }
    };
}

impl_35!()