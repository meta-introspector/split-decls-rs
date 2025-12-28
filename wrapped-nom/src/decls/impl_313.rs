macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl AsBytes for str { # [inline (always)] fn as_bytes (& self) -> & [u8] { self . as_ref () } }
    };
}

impl_313!();