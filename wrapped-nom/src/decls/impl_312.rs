macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < 'a > AsBytes for & 'a str { # [inline (always)] fn as_bytes (& self) -> & [u8] { (* self) . as_bytes () } }
    };
}

impl_312!()