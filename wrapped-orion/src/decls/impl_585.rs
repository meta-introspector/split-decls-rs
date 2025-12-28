macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl Base for DHKEM_X25519_SHA256_CHACHA20 { }
    };
}

impl_585!()