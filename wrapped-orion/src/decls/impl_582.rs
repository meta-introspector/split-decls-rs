macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl Eq for DHKEM_X25519_SHA256_CHACHA20 { }
    };
}

impl_582!()