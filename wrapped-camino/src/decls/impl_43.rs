macro_rules! deps {
    () => {
        Utf8Components!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl AsRef < Path > for Utf8Components < '_ > { # [inline] fn as_ref (& self) -> & Path { self . as_path () . as_ref () } }
    };
}

impl_43!()