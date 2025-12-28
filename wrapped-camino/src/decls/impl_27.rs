macro_rules! deps {
    () => {
        Utf8Components!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl AsRef < str > for Utf8Components < '_ > { # [inline] fn as_ref (& self) -> & str { self . as_path () . as_ref () } }
    };
}

impl_27!()