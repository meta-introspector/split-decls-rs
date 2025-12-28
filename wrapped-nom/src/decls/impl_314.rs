macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < 'a > AsBytes for & 'a [u8] { # [inline (always)] fn as_bytes (& self) -> & [u8] { self } }
    };
}

impl_314!();