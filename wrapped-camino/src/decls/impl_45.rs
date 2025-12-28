macro_rules! deps {
    () => {
        Utf8Component!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl AsRef < str > for Utf8Component < '_ > { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_45!()