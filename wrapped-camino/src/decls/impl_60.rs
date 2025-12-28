macro_rules! deps {
    () => {
        Utf8Component!();
        Utf8Path!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for Utf8Component < '_ > { # [inline] fn as_ref (& self) -> & Utf8Path { self . as_str () . as_ref () } }
    };
}

impl_60!();