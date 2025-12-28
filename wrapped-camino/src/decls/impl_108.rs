macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl AsRef < str > for Utf8Path { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_108!()