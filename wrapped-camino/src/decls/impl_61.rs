macro_rules! deps {
    () => {
        Utf8Component!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl AsRef < Path > for Utf8Component < '_ > { # [inline] fn as_ref (& self) -> & Path { self . as_os_str () . as_ref () } }
    };
}

impl_61!();