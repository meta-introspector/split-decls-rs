macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl AsRef < Path > for Utf8Path { # [inline] fn as_ref (& self) -> & Path { & self . 0 } }
    };
}

impl_123!()