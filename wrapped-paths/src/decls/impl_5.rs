macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl AsRef < OsStr > for AbsPathBuf { fn as_ref (& self) -> & OsStr { self . 0 . as_ref () } }
    };
}

impl_5!();