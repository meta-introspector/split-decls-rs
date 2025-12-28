macro_rules! deps {
    () => {
        AsBytes!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < 'a , const N : usize > AsBytes for & 'a [u8 ; N] { # [inline (always)] fn as_bytes (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_316!()