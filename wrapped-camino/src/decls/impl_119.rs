macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for Utf8Path { # [inline] fn as_ref (& self) -> & Utf8Path { self } }
    };
}

impl_119!();