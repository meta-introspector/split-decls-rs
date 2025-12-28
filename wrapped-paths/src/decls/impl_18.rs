macro_rules! deps {
    () => {
        AbsPath!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl AsRef < OsStr > for AbsPath { fn as_ref (& self) -> & OsStr { self . 0 . as_ref () } }
    };
}

impl_18!();