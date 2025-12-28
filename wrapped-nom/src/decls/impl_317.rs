macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < const N : usize > AsBytes for [u8 ; N] { # [inline (always)] fn as_bytes (& self) -> & [u8] { self } }
    };
}

impl_317!();