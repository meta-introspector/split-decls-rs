macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8Components!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for Utf8Components < '_ > { # [inline] fn as_ref (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_42!()