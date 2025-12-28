macro_rules! deps {
    () => {
        RelPathBuf!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl AsRef < Path > for RelPathBuf { fn as_ref (& self) -> & Path { self . 0 . as_ref () } }
    };
}

impl_27!()