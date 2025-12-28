macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl AsRef < Path > for AbsPathBuf { fn as_ref (& self) -> & Path { self . 0 . as_ref () } }
    };
}

impl_6!();