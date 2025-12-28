macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl AsBytes for [u8] { # [inline (always)] fn as_bytes (& self) -> & [u8] { self } }
    };
}

impl_315!();