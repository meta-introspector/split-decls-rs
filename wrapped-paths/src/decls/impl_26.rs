macro_rules! deps {
    () => {
        RelPathBuf!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for RelPathBuf { fn as_ref (& self) -> & Utf8Path { self . 0 . as_path () } }
    };
}

impl_26!()