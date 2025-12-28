macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for AbsPathBuf { fn as_ref (& self) -> & Utf8Path { self . 0 . as_path () } }
    };
}

impl_4!()