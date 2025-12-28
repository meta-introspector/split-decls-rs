macro_rules! deps {
    () => {
        Iter!();
        Utf8Path!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for Iter < '_ > { # [inline] fn as_ref (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_49!();